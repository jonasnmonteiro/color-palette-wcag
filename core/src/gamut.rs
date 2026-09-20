use crate::spaces::{LinearRgb, Oklch, Rgb};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetGamut {
    Srgb,
    DisplayP3,
    Rec2020,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamutCusp {
    pub hue: f64,
    pub lightness: f64,
    pub max_chroma: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamutMappingResult {
    pub original: Oklch,
    pub mapped: Oklch,
    pub in_gamut: bool,
    pub iterations: usize,
    pub target_gamut: TargetGamut,
}

pub fn is_in_srgb_gamut(oklch: &Oklch) -> bool {
    let lin = LinearRgb::from_oklch(oklch);
    let eps = 1e-6;
    lin.r >= -eps && lin.r <= 1.0 + eps &&
    lin.g >= -eps && lin.g <= 1.0 + eps &&
    lin.b >= -eps && lin.b <= 1.0 + eps
}

pub fn is_in_display_p3_gamut(oklch: &Oklch) -> bool {
    let lin = LinearRgb::from_oklch(oklch);
    let p3_r = 0.8224621 * lin.r + 0.1775380 * lin.g + 0.0000000 * lin.b;
    let p3_g = 0.0331941 * lin.r + 0.9668058 * lin.g + 0.0000000 * lin.b;
    let p3_b = 0.0170827 * lin.r + 0.0723974 * lin.g + 0.9105199 * lin.b;
    let eps = 1e-6;
    p3_r >= -eps && p3_r <= 1.0 + eps &&
    p3_g >= -eps && p3_g <= 1.0 + eps &&
    p3_b >= -eps && p3_b <= 1.0 + eps
}

pub fn is_in_gamut(oklch: &Oklch, gamut: TargetGamut) -> bool {
    match gamut {
        TargetGamut::Srgb => is_in_srgb_gamut(oklch),
        TargetGamut::DisplayP3 => is_in_display_p3_gamut(oklch),
        TargetGamut::Rec2020 => is_in_srgb_gamut(oklch),
    }
}

pub fn find_gamut_cusp(hue: f64, gamut: TargetGamut) -> GamutCusp {
    let mut best_l = 0.5;
    let mut best_c = 0.0;

    for step in 1..99 {
        let l = step as f64 / 100.0;
        let mut low_c = 0.0;
        let mut high_c = 0.5;

        for _ in 0..16 {
            let mid_c = (low_c + high_c) * 0.5;
            let sample = Oklch::new(l, mid_c, hue);
            if is_in_gamut(&sample, gamut) {
                low_c = mid_c;
            } else {
                high_c = mid_c;
            }
        }

        if low_c > best_c {
            best_c = low_c;
            best_l = l;
        }
    }

    GamutCusp {
        hue,
        lightness: best_l,
        max_chroma: best_c,
    }
}

pub fn map_to_gamut_binary_search(
    oklch: &Oklch,
    gamut: TargetGamut,
    precision: f64,
) -> GamutMappingResult {
    if is_in_gamut(oklch, gamut) {
        return GamutMappingResult {
            original: *oklch,
            mapped: *oklch,
            in_gamut: true,
            iterations: 0,
            target_gamut: gamut,
        };
    }

    let mut low_c = 0.0;
    let mut high_c = oklch.c;
    let mut iterations = 0;

    while (high_c - low_c) > precision && iterations < 50 {
        let mid_c = (low_c + high_c) * 0.5;
        let candidate = Oklch::new(oklch.l, mid_c, oklch.h);
        if is_in_gamut(&candidate, gamut) {
            low_c = mid_c;
        } else {
            high_c = mid_c;
        }
        iterations += 1;
    }

    let mapped = Oklch::new(oklch.l, low_c, oklch.h);

    GamutMappingResult {
        original: *oklch,
        mapped,
        in_gamut: false,
        iterations,
        target_gamut: gamut,
    }
}

pub fn clip_to_srgb(oklch: &Oklch) -> Rgb {
    let result = map_to_gamut_binary_search(oklch, TargetGamut::Srgb, 1e-4);
    Rgb::from_oklch(&result.mapped)
}

use crate::spaces::{Oklch, Rgb};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HarmonyMode {
    Analogous,
    Complementary,
    SplitComplementary,
    Triadic,
    Tetradic,
    Square,
    Monochromatic,
    Custom,
}

impl HarmonyMode {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "analogous" | "analoga" => Some(Self::Analogous),
            "complementary" | "complementar" => Some(Self::Complementary),
            "split-complementary" | "complementar-dividida" => Some(Self::SplitComplementary),
            "triadic" | "triade" => Some(Self::Triadic),
            "tetradic" | "tetrade" => Some(Self::Tetradic),
            "square" | "quadrado" => Some(Self::Square),
            "monochromatic" | "monocromatica" => Some(Self::Monochromatic),
            "custom" | "personalizado" => Some(Self::Custom),
            _ => None,
        }
    }
}

pub fn generate_harmony(base: &Oklch, mode: HarmonyMode, count: usize) -> Vec<Oklch> {
    if count == 0 {
        return Vec::new();
    }

    if mode == HarmonyMode::Monochromatic {
        let offsets = match count {
            1 => vec![0.0],
            3 => vec![0.15, 0.0, -0.15],
            5 => vec![0.20, 0.10, 0.0, -0.10, -0.20],
            _ => {
                let step = 0.40 / (count as f64 - 1.0);
                (0..count)
                    .map(|i| 0.20 - (i as f64 * step))
                    .collect::<Vec<f64>>()
            }
        };

        return offsets
            .into_iter()
            .map(|dl| {
                Oklch::new(
                    (base.l + dl).clamp(0.08, 0.94),
                    base.c,
                    base.h,
                )
            })
            .collect();
    }

    let offsets: Vec<f64> = match mode {
        HarmonyMode::Analogous => vec![-30.0, -15.0, 0.0, 15.0, 30.0],
        HarmonyMode::Complementary => vec![0.0, 180.0],
        HarmonyMode::SplitComplementary => vec![0.0, 150.0, 210.0],
        HarmonyMode::Triadic => vec![0.0, 120.0, 240.0],
        HarmonyMode::Tetradic => vec![0.0, 60.0, 180.0, 240.0],
        HarmonyMode::Square => vec![0.0, 90.0, 180.0, 270.0],
        HarmonyMode::Custom => vec![0.0, 72.0, 144.0, 216.0, 288.0],
        HarmonyMode::Monochromatic => unreachable!(),
    };

    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let off = offsets[i % offsets.len()];
        let cycle = (i / offsets.len()) as f64;
        let h = (base.h + off + 360.0) % 360.0;
        let l = (base.l - cycle * 0.10).clamp(0.08, 0.94);
        out.push(Oklch::new(l, base.c, h));
    }

    out
}

pub fn generate_harmony_hexes(base_hex: &str, mode: HarmonyMode, count: usize) -> Vec<String> {
    let rgb = match Rgb::from_hex(base_hex) {
        Some(r) => r,
        None => return Vec::new(),
    };
    let oklch = rgb.to_oklch();
    generate_harmony(&oklch, mode, count)
        .into_iter()
        .map(|c| c.to_hex())
        .collect()
}

use crate::contrast::{apca_contrast, wcag21_contrast, ApcaVerdict, WcagVerdict};
use crate::harmonies::{generate_harmony, HarmonyMode};
use crate::spaces::{Oklch, Rgb};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Swatch {
    pub hex: String,
    pub rgb: Rgb,
    pub oklch: Oklch,
    pub wcag_on_white: WcagVerdict,
    pub wcag_on_black: WcagVerdict,
    pub apca_on_white: ApcaVerdict,
    pub apca_on_black: ApcaVerdict,
}

impl Swatch {
    pub fn from_oklch(oklch: Oklch) -> Self {
        let rgb = oklch.to_rgb();
        let hex = rgb.to_hex();
        let white = Rgb::new(255, 255, 255);
        let black = Rgb::new(0, 0, 0);

        Self {
            hex,
            rgb,
            oklch,
            wcag_on_white: wcag21_contrast(&rgb, &white),
            wcag_on_black: wcag21_contrast(&rgb, &black),
            apca_on_white: apca_contrast(&rgb, &white),
            apca_on_black: apca_contrast(&rgb, &black),
        }
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        let rgb = Rgb::from_hex(hex)?;
        let oklch = rgb.to_oklch();
        Some(Self::from_oklch(oklch))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Palette {
    pub base_hex: String,
    pub mode: HarmonyMode,
    pub swatches: Vec<Swatch>,
}

impl Palette {
    pub fn generate(base_hex: &str, mode: HarmonyMode, count: usize) -> Option<Self> {
        let base_rgb = Rgb::from_hex(base_hex)?;
        let base_oklch = base_rgb.to_oklch();
        let oklch_list = generate_harmony(&base_oklch, mode, count);
        let swatches = oklch_list.into_iter().map(Swatch::from_oklch).collect();

        Some(Self {
            base_hex: base_rgb.to_hex(),
            mode,
            swatches,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignSystemTokens {
    pub light: BTreeMap<String, String>,
    pub dark: BTreeMap<String, String>,
}

impl DesignSystemTokens {
    pub fn from_seed_hex(seed_hex: &str) -> Option<Self> {
        let base_rgb = Rgb::from_hex(seed_hex)?;
        let base_ok = base_rgb.to_oklch();

        let mut light = BTreeMap::new();
        let mut dark = BTreeMap::new();

        let brand = base_rgb.to_hex();
        let brand_hover = Oklch::new((base_ok.l - 0.08).clamp(0.08, 0.94), base_ok.c, base_ok.h).to_hex();
        let deep = Oklch::new((base_ok.l - 0.18).clamp(0.08, 0.94), base_ok.c * 0.9, base_ok.h).to_hex();
        let bg_light = Oklch::new(0.98, (base_ok.c * 0.1).min(0.03), base_ok.h).to_hex();
        let surface_light = Oklch::new(0.94, (base_ok.c * 0.1).min(0.04), base_ok.h).to_hex();
        let ink_light = Oklch::new(0.15, (base_ok.c * 0.2).min(0.05), base_ok.h).to_hex();
        let muted_light = Oklch::new(0.45, (base_ok.c * 0.15).min(0.04), base_ok.h).to_hex();
        let line_light = Oklch::new(0.88, (base_ok.c * 0.1).min(0.03), base_ok.h).to_hex();

        light.insert("--brand".to_string(), brand);
        light.insert("--brand-hover".to_string(), brand_hover);
        light.insert("--deep".to_string(), deep);
        light.insert("--bg".to_string(), bg_light);
        light.insert("--surface".to_string(), "#FFFFFF".to_string());
        light.insert("--surface-2".to_string(), surface_light);
        light.insert("--ink".to_string(), ink_light);
        light.insert("--muted".to_string(), muted_light);
        light.insert("--line".to_string(), line_light);

        let brand_dark = Oklch::new((base_ok.l + 0.12).clamp(0.45, 0.85), base_ok.c, base_ok.h).to_hex();
        let brand_dark_hover = Oklch::new((base_ok.l + 0.18).clamp(0.50, 0.90), base_ok.c, base_ok.h).to_hex();
        let bg_dark = Oklch::new(0.08, (base_ok.c * 0.15).min(0.04), base_ok.h).to_hex();
        let surface_dark = Oklch::new(0.14, (base_ok.c * 0.15).min(0.04), base_ok.h).to_hex();
        let surface_2_dark = Oklch::new(0.19, (base_ok.c * 0.15).min(0.04), base_ok.h).to_hex();
        let ink_dark = Oklch::new(0.94, (base_ok.c * 0.1).min(0.03), base_ok.h).to_hex();
        let muted_dark = Oklch::new(0.68, (base_ok.c * 0.12).min(0.04), base_ok.h).to_hex();
        let line_dark = Oklch::new(0.24, (base_ok.c * 0.15).min(0.04), base_ok.h).to_hex();

        dark.insert("--brand".to_string(), brand_dark);
        dark.insert("--brand-hover".to_string(), brand_dark_hover);
        dark.insert("--bg".to_string(), bg_dark);
        dark.insert("--surface".to_string(), surface_dark);
        dark.insert("--surface-2".to_string(), surface_2_dark);
        dark.insert("--ink".to_string(), ink_dark);
        dark.insert("--muted".to_string(), muted_dark);
        dark.insert("--line".to_string(), line_dark);

        Some(Self { light, dark })
    }
}

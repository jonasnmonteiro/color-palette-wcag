pub mod contrast;
pub mod cvd;
pub mod delta_e;
pub mod exporter;
pub mod gamut;
pub mod harmonies;
pub mod palette;
pub mod quantization;
pub mod spaces;

pub use contrast::{
    apca_contrast, best_ink_for_ground, relative_luminance, wcag21_contrast, ApcaVerdict,
    WcagVerdict,
};
pub use cvd::{audit_cvd_contrast, simulate_cvd, CvdAuditResult, CvdType};
pub use delta_e::ciede2000;
pub use exporter::{ExportFormat, TokenExporter};
pub use gamut::{
    clip_to_srgb, find_gamut_cusp, is_in_display_p3_gamut, is_in_gamut, is_in_srgb_gamut,
    map_to_gamut_binary_search, GamutCusp, GamutMappingResult, TargetGamut,
};
pub use harmonies::{generate_harmony, generate_harmony_hexes, HarmonyMode};
pub use palette::{DesignSystemTokens, Palette, Swatch};
pub use quantization::{quantize_image_oklab, QuantizedColor};
pub use spaces::{Cielab, Hsl, Hsv, LinearRgb, Oklab, Oklch, Rgb};

pub fn hex_to_oklch(hex: &str) -> Option<Oklch> {
    Rgb::from_hex(hex).map(|rgb| rgb.to_oklch())
}

pub fn oklch_to_hex(l: f64, c: f64, h: f64) -> String {
    Oklch::new(l, c, h).to_hex()
}

pub fn perceptual_distance(hex1: &str, hex2: &str) -> Option<f64> {
    let rgb1 = Rgb::from_hex(hex1)?;
    let rgb2 = Rgb::from_hex(hex2)?;
    let lab1 = rgb1.to_cielab();
    let lab2 = rgb2.to_cielab();
    Some(ciede2000(&lab1, &lab2))
}

pub fn audit_contrast_hex(fg_hex: &str, bg_hex: &str) -> Option<(WcagVerdict, ApcaVerdict)> {
    let fg = Rgb::from_hex(fg_hex)?;
    let bg = Rgb::from_hex(bg_hex)?;
    Some((wcag21_contrast(&fg, &bg), apca_contrast(&fg, &bg)))
}

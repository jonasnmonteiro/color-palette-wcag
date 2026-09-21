use crate::contrast::{apca_contrast, wcag21_contrast, ApcaVerdict, WcagVerdict};
use crate::spaces::Rgb;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CvdType {
    Protanopia,
    Deuteranopia,
    Tritanopia,
    Achromatopsia,
}

impl CvdType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "protanopia" | "protan" => Some(Self::Protanopia),
            "deuteranopia" | "deutan" => Some(Self::Deuteranopia),
            "tritanopia" | "tritan" => Some(Self::Tritanopia),
            "achromatopsia" | "mono" | "grayscale" => Some(Self::Achromatopsia),
            _ => None,
        }
    }
}

pub fn simulate_cvd(rgb: &Rgb, cvd: CvdType, severity: f64) -> Rgb {
    let sev = severity.clamp(0.0, 1.0);
    let r = rgb.r as f64 / 255.0;
    let g = rgb.g as f64 / 255.0;
    let b = rgb.b as f64 / 255.0;

    let (sim_r, sim_g, sim_b) = match cvd {
        CvdType::Protanopia => (
            0.56667 * r + 0.43333 * g,
            0.55833 * r + 0.44167 * g,
            0.24167 * g + 0.75833 * b,
        ),
        CvdType::Deuteranopia => (
            0.625 * r + 0.375 * g,
            0.700 * r + 0.300 * g,
            0.300 * g + 0.700 * b,
        ),
        CvdType::Tritanopia => (
            0.950 * r + 0.050 * g,
            0.43333 * g + 0.56667 * b,
            0.475 * g + 0.525 * b,
        ),
        CvdType::Achromatopsia => {
            let y = 0.2126 * r + 0.7152 * g + 0.0722 * b;
            (y, y, y)
        }
    };

    let final_r = (1.0 - sev) * r + sev * sim_r;
    let final_g = (1.0 - sev) * g + sev * sim_g;
    let final_b = (1.0 - sev) * b + sev * sim_b;

    Rgb::new(
        (final_r.clamp(0.0, 1.0) * 255.0).round() as u8,
        (final_g.clamp(0.0, 1.0) * 255.0).round() as u8,
        (final_b.clamp(0.0, 1.0) * 255.0).round() as u8,
    )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CvdAuditResult {
    pub cvd_type: CvdType,
    #[serde(serialize_with = "rgb_as_hex", deserialize_with = "rgb_from_hex")]
    pub simulated_fg: Rgb,
    #[serde(serialize_with = "rgb_as_hex", deserialize_with = "rgb_from_hex")]
    pub simulated_bg: Rgb,
    pub wcag: WcagVerdict,
    pub apca: ApcaVerdict,
}

fn rgb_as_hex<S: serde::Serializer>(rgb: &Rgb, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&rgb.to_hex())
}

fn rgb_from_hex<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Rgb, D::Error> {
    let hex = String::deserialize(deserializer)?;
    Rgb::from_hex(&hex).ok_or_else(|| serde::de::Error::custom("invalid hex colour"))
}

pub fn audit_cvd_contrast(fg: &Rgb, bg: &Rgb, cvd: CvdType, severity: f64) -> CvdAuditResult {
    let sim_fg = simulate_cvd(fg, cvd, severity);
    let sim_bg = simulate_cvd(bg, cvd, severity);
    let wcag = wcag21_contrast(&sim_fg, &sim_bg);
    let apca = apca_contrast(&sim_fg, &sim_bg);

    CvdAuditResult {
        cvd_type: cvd,
        simulated_fg: sim_fg,
        simulated_bg: sim_bg,
        wcag,
        apca,
    }
}

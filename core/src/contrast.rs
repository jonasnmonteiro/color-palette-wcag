use crate::spaces::{LinearRgb, Rgb};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WcagVerdict {
    pub ratio: f64,
    pub normal_aa: bool,
    pub large_aa: bool,
    pub normal_aaa: bool,
    pub large_aaa: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ApcaVerdict {
    pub lc: f64,
    pub is_dark_on_light: bool,
    pub body_text: bool,
    pub fluent_text: bool,
    pub large_text: bool,
    pub spot_text: bool,
    pub non_text: bool,
}

pub fn relative_luminance(rgb: &Rgb) -> f64 {
    let lin = rgb.to_linear();
    0.2126729 * lin.r + 0.7151522 * lin.g + 0.0721750 * lin.b
}

pub fn wcag21_contrast(fg: &Rgb, bg: &Rgb) -> WcagVerdict {
    let l1 = relative_luminance(fg);
    let l2 = relative_luminance(bg);

    let lighter = l1.max(l2);
    let darker = l1.min(l2);

    let ratio = (lighter + 0.05) / (darker + 0.05);

    WcagVerdict {
        ratio,
        normal_aa: ratio >= 4.5,
        large_aa: ratio >= 3.0,
        normal_aaa: ratio >= 7.0,
        large_aaa: ratio >= 4.5,
    }
}

pub fn apca_contrast(text: &Rgb, background: &Rgb) -> ApcaVerdict {
    let y_txt = apca_y(text);
    let y_bg = apca_y(background);

    let abs_diff = (y_txt - y_bg).abs();
    if abs_diff < 0.0005 {
        return ApcaVerdict {
            lc: 0.0,
            is_dark_on_light: false,
            body_text: false,
            fluent_text: false,
            large_text: false,
            spot_text: false,
            non_text: false,
        };
    }

    let is_dark_on_light = y_bg > y_txt;
    let lc = if is_dark_on_light {
        let s_txt = y_txt.powf(0.56);
        let s_bg = y_bg.powf(0.65);
        -(s_bg - s_txt) * 1.141445 * 100.0
    } else {
        let s_txt = y_txt.powf(0.62);
        let s_bg = y_bg.powf(0.57);
        (s_txt - s_bg) * 1.141445 * 100.0
    };

    let abs_lc = lc.abs();

    ApcaVerdict {
        lc,
        is_dark_on_light,
        body_text: abs_lc >= 75.0,
        fluent_text: abs_lc >= 60.0,
        large_text: abs_lc >= 45.0,
        spot_text: abs_lc >= 30.0,
        non_text: abs_lc >= 15.0,
    }
}

pub fn best_ink_for_ground(background: &Rgb, candidate_a: &Rgb, candidate_b: &Rgb) -> Rgb {
    let ratio_a = wcag21_contrast(candidate_a, background).ratio;
    let ratio_b = wcag21_contrast(candidate_b, background).ratio;
    if ratio_a >= ratio_b {
        *candidate_a
    } else {
        *candidate_b
    }
}

fn apca_y(rgb: &Rgb) -> f64 {
    let lin = LinearRgb {
        r: apca_linearize(rgb.r as f64 / 255.0),
        g: apca_linearize(rgb.g as f64 / 255.0),
        b: apca_linearize(rgb.b as f64 / 255.0),
        a: rgb.a,
    };

    let y = 0.2126729 * lin.r + 0.7151522 * lin.g + 0.0721750 * lin.b;
    if y < 0.022 {
        y + (0.022 - y).powf(1.414)
    } else {
        y
    }
}

#[inline]
fn apca_linearize(v: f64) -> f64 {
    v.powf(2.4)
}

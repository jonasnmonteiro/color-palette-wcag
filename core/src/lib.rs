pub mod delta_e;
pub mod spaces;

pub use delta_e::ciede2000;
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

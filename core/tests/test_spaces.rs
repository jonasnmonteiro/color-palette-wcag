use auracolor_core::spaces::{Cielab, Hsl, Hsv, Oklch, Rgb};
use auracolor_core::{ciede2000, hex_to_oklch, oklch_to_hex, perceptual_distance};

#[test]
fn test_hex_parsing() {
    let rgb_3 = Rgb::from_hex("#FFF").unwrap();
    assert_eq!(rgb_3, Rgb::new(255, 255, 255));

    let rgb_6 = Rgb::from_hex("#4F46E5").unwrap();
    assert_eq!(rgb_6, Rgb::new(79, 70, 229));

    let rgb_8 = Rgb::from_hex("#10B98180").unwrap();
    assert_eq!(rgb_8.r, 16);
    assert_eq!(rgb_8.g, 185);
    assert_eq!(rgb_8.b, 129);
    assert!((rgb_8.a - 0.5019).abs() < 0.01);
}

#[test]
fn test_oklch_roundtrip_white_black() {
    let white = Rgb::new(255, 255, 255);
    let oklch_white = white.to_oklch();
    assert!((oklch_white.l - 1.0).abs() < 0.01);
    assert!(oklch_white.c < 0.01);
    let recovered_white = oklch_white.to_rgb();
    assert_eq!(recovered_white.r, 255);
    assert_eq!(recovered_white.g, 255);
    assert_eq!(recovered_white.b, 255);

    let black = Rgb::new(0, 0, 0);
    let oklch_black = black.to_oklch();
    assert!(oklch_black.l < 0.001);
    assert!(oklch_black.c < 0.001);
    let recovered_black = oklch_black.to_rgb();
    assert_eq!(recovered_black.r, 0);
    assert_eq!(recovered_black.g, 0);
    assert_eq!(recovered_black.b, 0);
}

#[test]
fn test_oklch_roundtrip_primaries() {
    let primaries = ["#FF0000", "#00FF00", "#0000FF", "#FFFF00", "#00FFFF", "#FF00FF"];
    for hex in primaries {
        let rgb = Rgb::from_hex(hex).unwrap();
        let oklch = rgb.to_oklch();
        let recovered = oklch.to_rgb();
        assert_eq!(rgb.r, recovered.r);
        assert_eq!(rgb.g, recovered.g);
        assert_eq!(rgb.b, recovered.b);
    }
}

#[test]
fn test_ciede2000_properties() {
    let c1 = Cielab { l: 50.0, a: 2.6772, b: -79.7751, alpha: 1.0 };
    let c2 = Cielab { l: 50.0, a: 0.0, b: -82.7485, alpha: 1.0 };
    let d_same = ciede2000(&c1, &c1);
    assert!(d_same < 0.0001);

    let d_diff = ciede2000(&c1, &c2);
    assert!(d_diff > 1.0);
    assert!(d_diff < 3.0);

    let hex_dist = perceptual_distance("#000000", "#FFFFFF").unwrap();
    assert!(hex_dist > 90.0);
}

#[test]
fn test_hsl_and_hsv_conversions() {
    let rgb = Rgb::new(255, 128, 0);
    let hsl = rgb.to_hsl();
    assert!((hsl.h - 30.1).abs() < 1.0);
    assert!((hsl.s - 100.0).abs() < 1.0);
    assert!((hsl.l - 50.0).abs() < 1.0);

    let hsv = rgb.to_hsv();
    assert!((hsv.h - 30.1).abs() < 1.0);
    assert!((hsv.s - 100.0).abs() < 1.0);
    assert!((hsv.v - 100.0).abs() < 1.0);
}

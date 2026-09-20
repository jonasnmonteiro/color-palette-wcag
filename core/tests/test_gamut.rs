use colorust_core::{
    clip_to_srgb, find_gamut_cusp, is_in_display_p3_gamut, is_in_srgb_gamut,
    map_to_gamut_binary_search, Oklch, Rgb, TargetGamut,
};

#[test]
fn test_srgb_gamut_in_bounds() {
    let gray = Oklch::new(0.5, 0.0, 0.0);
    assert!(is_in_srgb_gamut(&gray));

    let white = Oklch::new(1.0, 0.0, 0.0);
    assert!(is_in_srgb_gamut(&white));

    let black = Oklch::new(0.0, 0.0, 0.0);
    assert!(is_in_srgb_gamut(&black));

    let blue = Rgb::new(59, 130, 246).to_oklch();
    assert!(is_in_srgb_gamut(&blue));
}

#[test]
fn test_out_of_gamut_detection() {
    let extreme_chroma = Oklch::new(0.7, 0.45, 140.0);
    assert!(!is_in_srgb_gamut(&extreme_chroma));
}

#[test]
fn test_binary_search_clipping() {
    let out_color = Oklch::new(0.65, 0.40, 150.0);
    assert!(!is_in_srgb_gamut(&out_color));

    let result = map_to_gamut_binary_search(&out_color, TargetGamut::Srgb, 1e-4);
    assert!(!result.in_gamut);
    assert!(is_in_srgb_gamut(&result.mapped));
    assert!(result.mapped.c < out_color.c);
    assert_eq!(result.mapped.l, out_color.l);
    assert_eq!(result.mapped.h, out_color.h);
}

#[test]
fn test_gamut_cusp_calculation() {
    let cusp = find_gamut_cusp(142.0, TargetGamut::Srgb);
    assert!(cusp.max_chroma > 0.15);
    assert!(cusp.lightness > 0.4 && cusp.lightness < 0.9);
}

#[test]
fn test_clip_to_srgb_valid_rgb() {
    let out_color = Oklch::new(0.75, 0.35, 120.0);
    let rgb = clip_to_srgb(&out_color);
    assert!(rgb.r <= 255);
    assert!(rgb.g <= 255);
    assert!(rgb.b <= 255);
}

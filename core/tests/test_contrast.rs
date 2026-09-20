use colorust_core::contrast::{
    apca_contrast, best_ink_for_ground, relative_luminance, wcag21_contrast,
};
use colorust_core::harmonies::{generate_harmony_hexes, HarmonyMode};
use colorust_core::palette::{DesignSystemTokens, Palette};
use colorust_core::spaces::Rgb;

#[test]
fn test_wcag21_black_white() {
    let black = Rgb::new(0, 0, 0);
    let white = Rgb::new(255, 255, 255);

    assert_eq!(relative_luminance(&black), 0.0);
    assert!((relative_luminance(&white) - 1.0).abs() < 0.001);

    let verdict = wcag21_contrast(&black, &white);
    assert!((verdict.ratio - 21.0).abs() < 0.1);
    assert!(verdict.normal_aa);
    assert!(verdict.large_aa);
    assert!(verdict.normal_aaa);
    assert!(verdict.large_aaa);

    let verdict_same = wcag21_contrast(&white, &white);
    assert!((verdict_same.ratio - 1.0).abs() < 0.001);
    assert!(!verdict_same.normal_aa);
}

#[test]
fn test_apca_polarity_and_levels() {
    let dark_text = Rgb::new(17, 24, 39);
    let light_bg = Rgb::new(249, 250, 251);

    let apca_dark_on_light = apca_contrast(&dark_text, &light_bg);
    assert!(apca_dark_on_light.is_dark_on_light);
    assert!(apca_dark_on_light.lc < 0.0);
    assert!(apca_dark_on_light.body_text);
    assert!(apca_dark_on_light.fluent_text);

    let light_text = Rgb::new(249, 250, 251);
    let dark_bg = Rgb::new(17, 24, 39);

    let apca_light_on_dark = apca_contrast(&light_text, &dark_bg);
    assert!(!apca_light_on_dark.is_dark_on_light);
    assert!(apca_light_on_dark.lc > 0.0);
    assert!(apca_light_on_dark.body_text);
}

#[test]
fn test_best_ink() {
    let dark_bg = Rgb::new(10, 15, 30);
    let light_ink = Rgb::new(255, 255, 255);
    let dark_ink = Rgb::new(20, 20, 20);

    let chosen = best_ink_for_ground(&dark_bg, &light_ink, &dark_ink);
    assert_eq!(chosen, light_ink);

    let light_bg = Rgb::new(250, 250, 250);
    let chosen_light = best_ink_for_ground(&light_bg, &light_ink, &dark_ink);
    assert_eq!(chosen_light, dark_ink);
}

#[test]
fn test_harmonies_generation() {
    let base = "#4F46E5";

    let modes = [
        HarmonyMode::Analogous,
        HarmonyMode::Complementary,
        HarmonyMode::SplitComplementary,
        HarmonyMode::Triadic,
        HarmonyMode::Tetradic,
        HarmonyMode::Square,
        HarmonyMode::Monochromatic,
        HarmonyMode::Custom,
    ];

    for mode in modes {
        let hexes = generate_harmony_hexes(base, mode, 5);
        assert_eq!(hexes.len(), 5);
        for h in hexes {
            assert!(h.starts_with('#'));
            assert_eq!(h.len(), 7);
        }
    }
}

#[test]
fn test_palette_and_design_system_tokens() {
    let pal = Palette::generate("#10B981", HarmonyMode::Triadic, 5).unwrap();
    assert_eq!(pal.swatches.len(), 5);
    assert_eq!(pal.base_hex, "#10B981");

    let tokens = DesignSystemTokens::from_seed_hex("#10B981").unwrap();
    assert!(tokens.light.contains_key("--brand"));
    assert!(tokens.light.contains_key("--bg"));
    assert!(tokens.light.contains_key("--surface"));
    assert!(tokens.dark.contains_key("--brand"));
    assert!(tokens.dark.contains_key("--surface"));
}

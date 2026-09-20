use colorust_core::contrast::{
    apca_contrast, best_ink_for_ground, wcag21_contrast,
};
use colorust_core::cvd::{audit_cvd_contrast, simulate_cvd, CvdType};
use colorust_core::delta_e::ciede2000;
use colorust_core::harmonies::{generate_harmony, HarmonyMode};
use colorust_core::palette::DesignSystemTokens;
use colorust_core::spaces::{Oklch, Rgb};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hex_to_oklch_wasm(hex: &str) -> Result<JsValue, JsValue> {
    let rgb = Rgb::from_hex(hex)
        .ok_or_else(|| JsValue::from_str("Invalid hex color"))?;
    let oklch = rgb.to_oklch();
    serde_wasm_bindgen::to_value(&oklch)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn oklch_to_hex_wasm(l: f64, c: f64, h: f64) -> String {
    Oklch::new(l, c, h).to_hex()
}

#[wasm_bindgen]
pub fn audit_contrast_wasm(fg_hex: &str, bg_hex: &str) -> Result<JsValue, JsValue> {
    let fg = Rgb::from_hex(fg_hex)
        .ok_or_else(|| JsValue::from_str("Invalid foreground hex"))?;
    let bg = Rgb::from_hex(bg_hex)
        .ok_or_else(|| JsValue::from_str("Invalid background hex"))?;

    let wcag = wcag21_contrast(&fg, &bg);
    let apca = apca_contrast(&fg, &bg);

    #[derive(serde::Serialize)]
    struct AuditResult {
        wcag: colorust_core::contrast::WcagVerdict,
        apca: colorust_core::contrast::ApcaVerdict,
    }

    serde_wasm_bindgen::to_value(&AuditResult { wcag, apca })
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn simulate_cvd_wasm(hex: &str, cvd_type_str: &str, severity: f64) -> Result<String, JsValue> {
    let rgb = Rgb::from_hex(hex)
        .ok_or_else(|| JsValue::from_str("Invalid hex color"))?;
    let cvd = CvdType::from_str(cvd_type_str)
        .ok_or_else(|| JsValue::from_str("Invalid CVD type"))?;

    let sim = simulate_cvd(&rgb, cvd, severity);
    Ok(sim.to_hex())
}

#[wasm_bindgen]
pub fn audit_cvd_contrast_wasm(
    fg_hex: &str,
    bg_hex: &str,
    cvd_type_str: &str,
    severity: f64,
) -> Result<JsValue, JsValue> {
    let fg = Rgb::from_hex(fg_hex)
        .ok_or_else(|| JsValue::from_str("Invalid foreground hex"))?;
    let bg = Rgb::from_hex(bg_hex)
        .ok_or_else(|| JsValue::from_str("Invalid background hex"))?;
    let cvd = CvdType::from_str(cvd_type_str)
        .ok_or_else(|| JsValue::from_str("Invalid CVD type"))?;

    let res = audit_cvd_contrast(&fg, &bg, cvd, severity);
    serde_wasm_bindgen::to_value(&res)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn ciede2000_distance_wasm(hex1: &str, hex2: &str) -> Result<f64, JsValue> {
    let rgb1 = Rgb::from_hex(hex1)
        .ok_or_else(|| JsValue::from_str("Invalid first hex"))?;
    let rgb2 = Rgb::from_hex(hex2)
        .ok_or_else(|| JsValue::from_str("Invalid second hex"))?;

    let lab1 = rgb1.to_cielab();
    let lab2 = rgb2.to_cielab();
    Ok(ciede2000(&lab1, &lab2))
}

#[wasm_bindgen]
pub fn generate_harmony_wasm(base_hex: &str, mode_str: &str, count: usize) -> Result<JsValue, JsValue> {
    let rgb = Rgb::from_hex(base_hex)
        .ok_or_else(|| JsValue::from_str("Invalid base hex"))?;
    let mode = HarmonyMode::from_str(mode_str)
        .ok_or_else(|| JsValue::from_str("Invalid harmony mode"))?;

    let oklch_base = rgb.to_oklch();
    let swatches = generate_harmony(&oklch_base, mode, count);

    #[derive(serde::Serialize)]
    struct WasmSwatch {
        hex: String,
        l: f64,
        c: f64,
        h: f64,
    }

    let result: Vec<WasmSwatch> = swatches
        .into_iter()
        .map(|ok| WasmSwatch {
            hex: ok.to_hex(),
            l: ok.l,
            c: ok.c,
            h: ok.h,
        })
        .collect();

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn best_ink_wasm(bg_hex: &str, ink_a_hex: &str, ink_b_hex: &str) -> Result<String, JsValue> {
    let bg = Rgb::from_hex(bg_hex)
        .ok_or_else(|| JsValue::from_str("Invalid background hex"))?;
    let a = Rgb::from_hex(ink_a_hex)
        .ok_or_else(|| JsValue::from_str("Invalid ink A hex"))?;
    let b = Rgb::from_hex(ink_b_hex)
        .ok_or_else(|| JsValue::from_str("Invalid ink B hex"))?;

    Ok(best_ink_for_ground(&bg, &a, &b).to_hex())
}

#[wasm_bindgen]
pub fn generate_tokens_wasm(seed_hex: &str) -> Result<JsValue, JsValue> {
    let tokens = DesignSystemTokens::from_seed_hex(seed_hex)
        .ok_or_else(|| JsValue::from_str("Invalid seed hex"))?;

    serde_wasm_bindgen::to_value(&tokens)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn is_in_gamut_wasm(l: f64, c: f64, h: f64, gamut_str: &str) -> bool {
    let oklch = Oklch::new(l, c, h);
    let gamut = match gamut_str.to_lowercase().as_str() {
        "p3" | "display-p3" => colorust_core::gamut::TargetGamut::DisplayP3,
        "rec2020" => colorust_core::gamut::TargetGamut::Rec2020,
        _ => colorust_core::gamut::TargetGamut::Srgb,
    };
    colorust_core::gamut::is_in_gamut(&oklch, gamut)
}

#[wasm_bindgen]
pub fn map_to_gamut_wasm(
    l: f64,
    c: f64,
    h: f64,
    gamut_str: &str,
    precision: f64,
) -> Result<JsValue, JsValue> {
    let oklch = Oklch::new(l, c, h);
    let gamut = match gamut_str.to_lowercase().as_str() {
        "p3" | "display-p3" => colorust_core::gamut::TargetGamut::DisplayP3,
        "rec2020" => colorust_core::gamut::TargetGamut::Rec2020,
        _ => colorust_core::gamut::TargetGamut::Srgb,
    };
    let res = colorust_core::gamut::map_to_gamut_binary_search(&oklch, gamut, precision);
    serde_wasm_bindgen::to_value(&res)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn find_gamut_cusp_wasm(hue: f64, gamut_str: &str) -> Result<JsValue, JsValue> {
    let gamut = match gamut_str.to_lowercase().as_str() {
        "p3" | "display-p3" => colorust_core::gamut::TargetGamut::DisplayP3,
        "rec2020" => colorust_core::gamut::TargetGamut::Rec2020,
        _ => colorust_core::gamut::TargetGamut::Srgb,
    };
    let cusp = colorust_core::gamut::find_gamut_cusp(hue, gamut);
    serde_wasm_bindgen::to_value(&cusp)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn quantize_image_wasm(
    pixels_rgba: &[u8],
    k: usize,
    max_iterations: usize,
) -> Result<JsValue, JsValue> {
    let res = colorust_core::quantization::quantize_image_oklab(pixels_rgba, k, max_iterations);
    serde_wasm_bindgen::to_value(&res)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}



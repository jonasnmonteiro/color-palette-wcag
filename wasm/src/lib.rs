use auracolor_core::contrast::{
    apca_contrast, best_ink_for_ground, wcag21_contrast,
};
use auracolor_core::delta_e::ciede2000;
use auracolor_core::harmonies::{generate_harmony, HarmonyMode};
use auracolor_core::palette::DesignSystemTokens;
use auracolor_core::spaces::{Oklch, Rgb};
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
        wcag: auracolor_core::contrast::WcagVerdict,
        apca: auracolor_core::contrast::ApcaVerdict,
    }

    serde_wasm_bindgen::to_value(&AuditResult { wcag, apca })
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

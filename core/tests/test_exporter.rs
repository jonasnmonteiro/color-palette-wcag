use colorust_core::exporter::{ExportFormat, TokenExporter};
use colorust_core::palette::DesignSystemTokens;

#[test]
fn test_css_export() {
    let tokens = DesignSystemTokens::from_seed_hex("#3B82F6").unwrap();
    let css = TokenExporter::export_css(&tokens);

    assert!(css.contains(":root {"));
    assert!(css.contains("--brand: #3B82F6;"));
    assert!(css.contains("[data-theme=\"dark\"]"));
}

#[test]
fn test_figma_tokens_export() {
    let tokens = DesignSystemTokens::from_seed_hex("#3B82F6").unwrap();
    let json_str = TokenExporter::export_figma_tokens(&tokens);

    assert!(json_str.contains("\"$value\": \"#3B82F6\""));
    assert!(json_str.contains("\"$type\": \"color\""));
    assert!(json_str.contains("\"global\""));
}

#[test]
fn test_tailwind_v4_export() {
    let tokens = DesignSystemTokens::from_seed_hex("#3B82F6").unwrap();
    let tw = TokenExporter::export_tailwind_v4(&tokens);

    assert!(tw.contains("@theme {"));
    assert!(tw.contains("--color-brand: #3B82F6;"));
}

#[test]
fn test_swift_export() {
    let tokens = DesignSystemTokens::from_seed_hex("#3B82F6").unwrap();
    let swift = TokenExporter::export_swift(&tokens);

    assert!(swift.contains("import SwiftUI"));
    assert!(swift.contains("public static let brand = Color("));
}

#[test]
fn test_android_xml_export() {
    let tokens = DesignSystemTokens::from_seed_hex("#3B82F6").unwrap();
    let xml = TokenExporter::export_android_xml(&tokens);

    assert!(xml.contains("<?xml version=\"1.0\" encoding=\"utf-8\"?>"));
    assert!(xml.contains("<color name=\"brand\">#3B82F6</color>"));
}

#[test]
fn test_generic_export_enum() {
    let tokens = DesignSystemTokens::from_seed_hex("#3B82F6").unwrap();
    let scss = TokenExporter::export(&tokens, ExportFormat::Scss);
    assert!(scss.contains("$brand: #3B82F6;"));
}

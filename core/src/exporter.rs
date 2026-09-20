use crate::palette::DesignSystemTokens;
use serde_json::{json, Value};

pub enum ExportFormat {
    Css,
    Scss,
    TailwindV3,
    TailwindV4,
    FigmaTokens,
    StyleDictionary,
    Swift,
    AndroidXml,
}

pub struct TokenExporter;

impl TokenExporter {
    pub fn export_css(tokens: &DesignSystemTokens) -> String {
        let mut out = String::new();
        out.push_str(":root {\n");
        for (k, v) in &tokens.light {
            out.push_str(&format!("  {}: {};\n", k, v));
        }
        out.push_str("}\n\n[data-theme=\"dark\"], .dark {\n");
        for (k, v) in &tokens.dark {
            out.push_str(&format!("  {}: {};\n", k, v));
        }
        out.push_str("}\n");
        out
    }

    pub fn export_scss(tokens: &DesignSystemTokens) -> String {
        let mut out = String::new();
        out.push_str("// Light Theme\n");
        for (k, v) in &tokens.light {
            let var_name = k.trim_start_matches("--");
            out.push_str(&format!("${}: {};\n", var_name, v));
        }
        out.push_str("\n// Dark Theme\n");
        for (k, v) in &tokens.dark {
            let var_name = k.trim_start_matches("--");
            out.push_str(&format!("${}-dark: {};\n", var_name, v));
        }
        out
    }

    pub fn export_tailwind_v3(tokens: &DesignSystemTokens) -> String {
        let mut colors = String::new();
        for (k, _) in &tokens.light {
            let name = k.trim_start_matches("--");
            let js_name = name.replace('-', "_");
            colors.push_str(&format!("        '{}': 'var({})',\n", js_name, k));
        }

        format!(
            "module.exports = {{\n  theme: {{\n    extend: {{\n      colors: {{\n{}      }}\n    }}\n  }}\n}};\n",
            colors
        )
    }

    pub fn export_tailwind_v4(tokens: &DesignSystemTokens) -> String {
        let mut out = String::new();
        out.push_str("@theme {\n");
        for (k, v) in &tokens.light {
            let name = k.trim_start_matches("--");
            out.push_str(&format!("  --color-{}: {};\n", name, v));
        }
        out.push_str("}\n");
        out
    }

    pub fn export_figma_tokens(tokens: &DesignSystemTokens) -> String {
        let mut light_map = serde_json::Map::new();
        for (k, v) in &tokens.light {
            let name = k.trim_start_matches("--").to_string();
            light_map.insert(
                name,
                json!({
                    "$value": v,
                    "$type": "color"
                }),
            );
        }

        let mut dark_map = serde_json::Map::new();
        for (k, v) in &tokens.dark {
            let name = k.trim_start_matches("--").to_string();
            dark_map.insert(
                name,
                json!({
                    "$value": v,
                    "$type": "color"
                }),
            );
        }

        let root = json!({
            "global": {
                "light": light_map,
                "dark": dark_map
            }
        });

        serde_json::to_string_pretty(&root).unwrap_or_default()
    }

    pub fn export_style_dictionary(tokens: &DesignSystemTokens) -> String {
        let mut color_tree = serde_json::Map::new();
        for (k, v) in &tokens.light {
            let name = k.trim_start_matches("--").to_string();
            color_tree.insert(
                name,
                json!({
                    "value": v,
                    "type": "color"
                }),
            );
        }

        let root = json!({
            "color": color_tree
        });

        serde_json::to_string_pretty(&root).unwrap_or_default()
    }

    pub fn export_swift(tokens: &DesignSystemTokens) -> String {
        let mut out = String::new();
        out.push_str("import SwiftUI\n\npublic enum AppTheme {\n    public enum Colors {\n");
        for (k, v) in &tokens.light {
            let name = k.trim_start_matches("--").replace('-', "_");
            let clean_hex = v.trim_start_matches('#');
            if clean_hex.len() == 6 {
                let r = u8::from_str_radix(&clean_hex[0..2], 16).unwrap_or(0) as f64 / 255.0;
                let g = u8::from_str_radix(&clean_hex[2..4], 16).unwrap_or(0) as f64 / 255.0;
                let b = u8::from_str_radix(&clean_hex[4..6], 16).unwrap_or(0) as f64 / 255.0;
                out.push_str(&format!(
                    "        public static let {} = Color(red: {:.3}, green: {:.3}, blue: {:.3})\n",
                    name, r, g, b
                ));
            }
        }
        out.push_str("    }\n}\n");
        out
    }

    pub fn export_android_xml(tokens: &DesignSystemTokens) -> String {
        let mut out = String::new();
        out.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<resources>\n");
        for (k, v) in &tokens.light {
            let name = k.trim_start_matches("--").replace('-', "_");
            out.push_str(&format!("    <color name=\"{}\">{}</color>\n", name, v));
        }
        out.push_str("</resources>\n");
        out
    }

    pub fn export(tokens: &DesignSystemTokens, format: ExportFormat) -> String {
        match format {
            ExportFormat::Css => Self::export_css(tokens),
            ExportFormat::Scss => Self::export_scss(tokens),
            ExportFormat::TailwindV3 => Self::export_tailwind_v3(tokens),
            ExportFormat::TailwindV4 => Self::export_tailwind_v4(tokens),
            ExportFormat::FigmaTokens => Self::export_figma_tokens(tokens),
            ExportFormat::StyleDictionary => Self::export_style_dictionary(tokens),
            ExportFormat::Swift => Self::export_swift(tokens),
            ExportFormat::AndroidXml => Self::export_android_xml(tokens),
        }
    }
}

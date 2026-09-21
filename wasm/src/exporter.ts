import { DesignSystemTokens } from './types.js';

export type ExportFormat =
  | 'css'
  | 'scss'
  | 'tailwind-v3'
  | 'tailwind-v4'
  | 'figma-tokens'
  | 'style-dictionary'
  | 'swift'
  | 'android-xml';

export function exportCss(tokens: DesignSystemTokens): string {
  let out = ':root {\n';
  for (const [k, v] of Object.entries(tokens.light)) {
    out += `  ${k}: ${v};\n`;
  }
  out += '}\n\n[data-theme="dark"], .dark {\n';
  for (const [k, v] of Object.entries(tokens.dark)) {
    out += `  ${k}: ${v};\n`;
  }
  out += '}\n';
  return out;
}

export function exportScss(tokens: DesignSystemTokens): string {
  let out = '// Light Theme\n';
  for (const [k, v] of Object.entries(tokens.light)) {
    const varName = k.replace(/^--/, '');
    out += `$${varName}: ${v};\n`;
  }
  out += '\n// Dark Theme\n';
  for (const [k, v] of Object.entries(tokens.dark)) {
    const varName = k.replace(/^--/, '');
    out += `$${varName}-dark: ${v};\n`;
  }
  return out;
}

export function exportTailwindV3(tokens: DesignSystemTokens): string {
  let colors = '';
  for (const k of Object.keys(tokens.light)) {
    const name = k.replace(/^--/, '');
    const jsName = name.replace(/-/g, '_');
    colors += `        '${jsName}': 'var(${k})',\n`;
  }

  return `module.exports = {
  theme: {
    extend: {
      colors: {
${colors}      }
    }
  }
};
`;
}

export function exportTailwindV4(tokens: DesignSystemTokens): string {
  let out = '@theme {\n';
  for (const [k, v] of Object.entries(tokens.light)) {
    const name = k.replace(/^--/, '');
    out += `  --color-${name}: ${v};\n`;
  }
  out += '}\n';
  return out;
}

export function exportFigmaTokens(tokens: DesignSystemTokens): string {
  const lightMap: Record<string, { $value: string; $type: string }> = {};
  for (const [k, v] of Object.entries(tokens.light)) {
    const name = k.replace(/^--/, '');
    lightMap[name] = {
      $value: v,
      $type: 'color',
    };
  }

  const darkMap: Record<string, { $value: string; $type: string }> = {};
  for (const [k, v] of Object.entries(tokens.dark)) {
    const name = k.replace(/^--/, '');
    darkMap[name] = {
      $value: v,
      $type: 'color',
    };
  }

  const root = {
    global: {
      light: lightMap,
      dark: darkMap,
    },
  };

  return JSON.stringify(root, null, 2);
}

export function exportStyleDictionary(tokens: DesignSystemTokens): string {
  const colorTree: Record<string, { value: string; type: string }> = {};
  for (const [k, v] of Object.entries(tokens.light)) {
    const name = k.replace(/^--/, '');
    colorTree[name] = {
      value: v,
      type: 'color',
    };
  }

  const root = {
    color: colorTree,
  };

  return JSON.stringify(root, null, 2);
}

export function exportSwift(tokens: DesignSystemTokens): string {
  let out = 'import SwiftUI\n\npublic enum AppTheme {\n    public enum Colors {\n';
  for (const [k, v] of Object.entries(tokens.light)) {
    const name = k.replace(/^--/, '').replace(/-/g, '_');
    const cleanHex = v.replace(/^#/, '');
    if (cleanHex.length === 6) {
      const r = (parseInt(cleanHex.substring(0, 2), 16) / 255.0).toFixed(3);
      const g = (parseInt(cleanHex.substring(2, 4), 16) / 255.0).toFixed(3);
      const b = (parseInt(cleanHex.substring(4, 6), 16) / 255.0).toFixed(3);
      out += `        public static let ${name} = Color(red: ${r}, green: ${g}, blue: ${b})\n`;
    }
  }
  out += '    }\n}\n';
  return out;
}

export function exportAndroidXml(tokens: DesignSystemTokens): string {
  let out = '<?xml version="1.0" encoding="utf-8"?>\n<resources>\n';
  for (const [k, v] of Object.entries(tokens.light)) {
    const name = k.replace(/^--/, '').replace(/-/g, '_');
    out += `    <color name="${name}">${v}</color>\n`;
  }
  out += '</resources>\n';
  return out;
}

export function exportTokens(tokens: DesignSystemTokens, format: ExportFormat): string {
  switch (format) {
    case 'css':
      return exportCss(tokens);
    case 'scss':
      return exportScss(tokens);
    case 'tailwind-v3':
      return exportTailwindV3(tokens);
    case 'tailwind-v4':
      return exportTailwindV4(tokens);
    case 'figma-tokens':
      return exportFigmaTokens(tokens);
    case 'style-dictionary':
      return exportStyleDictionary(tokens);
    case 'swift':
      return exportSwift(tokens);
    case 'android-xml':
      return exportAndroidXml(tokens);
    default:
      return exportCss(tokens);
  }
}

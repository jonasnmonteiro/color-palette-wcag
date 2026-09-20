import {
  ApcaVerdict,
  ContrastAuditResult,
  DesignSystemTokens,
  HarmonyMode,
  HslColor,
  HsvColor,
  OklchColor,
  RgbColor,
  SwatchItem,
  WcagVerdict,
} from './types';

export * from './types';
export * from './exporter';

let wasmModule: any = null;

export async function initAuraColorWasm(wasmBinaryOrUrl?: string | ArrayBuffer): Promise<boolean> {
  try {
    if (typeof window !== 'undefined' && (window as any).auracolor_wasm) {
      wasmModule = (window as any).auracolor_wasm;
      return true;
    }
    return false;
  } catch {
    return false;
  }
}

export function parseHex(hex: string): RgbColor | null {
  const clean = hex.trim().replace(/^#/, '');
  if (clean.length === 3) {
    const r = parseInt(clean[0] + clean[0], 16);
    const g = parseInt(clean[1] + clean[1], 16);
    const b = parseInt(clean[2] + clean[2], 16);
    return isNaN(r) || isNaN(g) || isNaN(b) ? null : { r, g, b, a: 1.0 };
  }
  if (clean.length === 6) {
    const r = parseInt(clean.substring(0, 2), 16);
    const g = parseInt(clean.substring(2, 4), 16);
    const b = parseInt(clean.substring(4, 6), 16);
    return isNaN(r) || isNaN(g) || isNaN(b) ? null : { r, g, b, a: 1.0 };
  }
  if (clean.length === 8) {
    const r = parseInt(clean.substring(0, 2), 16);
    const g = parseInt(clean.substring(2, 4), 16);
    const b = parseInt(clean.substring(4, 6), 16);
    const a = parseInt(clean.substring(6, 8), 16) / 255.0;
    return isNaN(r) || isNaN(g) || isNaN(b) || isNaN(a) ? null : { r, g, b, a };
  }
  return null;
}

export function rgbToHex(rgb: RgbColor): string {
  const toHex = (n: number) => Math.round(Math.max(0, Math.min(255, n))).toString(16).padStart(2, '0');
  return `#${toHex(rgb.r)}${toHex(rgb.g)}${toHex(rgb.b)}`.toUpperCase();
}

function srgbToLinear(c: number): number {
  return c <= 0.04045 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4);
}

function linearToSrgb(c: number): number {
  return c <= 0.0031308 ? c * 12.92 : 1.055 * Math.pow(c, 1.0 / 2.4) - 0.055;
}

function cbrtSigned(x: number): number {
  return Math.sign(x) * Math.cbrt(Math.abs(x));
}

export function hexToOklch(hex: string): OklchColor | null {
  const rgb = parseHex(hex);
  if (!rgb) return null;

  const r = srgbToLinear(rgb.r / 255.0);
  const g = srgbToLinear(rgb.g / 255.0);
  const b = srgbToLinear(rgb.b / 255.0);

  const l = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
  const m = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
  const s = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;

  const l_ = cbrtSigned(l);
  const m_ = cbrtSigned(m);
  const s_ = cbrtSigned(s);

  const L = 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_;
  const a = 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_;
  const b_ = 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_;

  const C = Math.sqrt(a * a + b_ * b_);
  let H = (Math.atan2(b_, a) * 180.0) / Math.PI;
  if (H < 0) H += 360.0;

  return { l: L, c: C, h: H, alpha: rgb.a };
}

export function oklchToHex(l: number, c: number, h: number): string {
  const hRad = (h * Math.PI) / 180.0;
  const a_ = c * Math.cos(hRad);
  const b_ = c * Math.sin(hRad);

  const l_ = l + 0.3963377774 * a_ + 0.2158037573 * b_;
  const m_ = l - 0.1055613458 * a_ - 0.0638541728 * b_;
  const s_ = l - 0.0894841775 * a_ - 1.2914855480 * b_;

  const lLin = l_ * l_ * l_;
  const mLin = m_ * m_ * m_;
  const sLin = s_ * s_ * s_;

  const rLin = 4.0767416621 * lLin - 3.3077115913 * mLin + 0.2309699292 * sLin;
  const gLin = -1.2684380046 * lLin + 2.6097574011 * mLin - 0.3413193965 * sLin;
  const bLin = -0.0041960863 * lLin - 0.7034186147 * mLin + 1.7076147010 * sLin;

  const r = Math.round(Math.max(0, Math.min(255, linearToSrgb(rLin) * 255.0)));
  const g = Math.round(Math.max(0, Math.min(255, linearToSrgb(gLin) * 255.0)));
  const b = Math.round(Math.max(0, Math.min(255, linearToSrgb(bLin) * 255.0)));

  return rgbToHex({ r, g, b });
}

export function relativeLuminance(rgb: RgbColor): number {
  const r = srgbToLinear(rgb.r / 255.0);
  const g = srgbToLinear(rgb.g / 255.0);
  const b = srgbToLinear(rgb.b / 255.0);
  return 0.2126729 * r + 0.7151522 * g + 0.0721750 * b;
}

export function auditWcag21(fg: RgbColor, bg: RgbColor): WcagVerdict {
  const l1 = relativeLuminance(fg);
  const l2 = relativeLuminance(bg);
  const lighter = Math.max(l1, l2);
  const darker = Math.min(l1, l2);
  const ratio = (lighter + 0.05) / (darker + 0.05);

  return {
    ratio,
    normal_aa: ratio >= 4.5,
    large_aa: ratio >= 3.0,
    normal_aaa: ratio >= 7.0,
    large_aaa: ratio >= 4.5,
  };
}

function apcaY(rgb: RgbColor): number {
  const r = Math.pow(rgb.r / 255.0, 2.4);
  const g = Math.pow(rgb.g / 255.0, 2.4);
  const b = Math.pow(rgb.b / 255.0, 2.4);
  let y = 0.2126729 * r + 0.7151522 * g + 0.0721750 * b;
  if (y < 0.022) {
    y += Math.pow(0.022 - y, 1.414);
  }
  return y;
}

export function auditApca(text: RgbColor, background: RgbColor): ApcaVerdict {
  const yTxt = apcaY(text);
  const yBg = apcaY(background);

  const absDiff = Math.abs(yTxt - yBg);
  if (absDiff < 0.0005) {
    return {
      lc: 0.0,
      is_dark_on_light: false,
      body_text: false,
      fluent_text: false,
      large_text: false,
      spot_text: false,
      non_text: false,
    };
  }

  const isDarkOnLight = yBg > yTxt;
  const lc = isDarkOnLight
    ? -(Math.pow(yBg, 0.65) - Math.pow(yTxt, 0.56)) * 1.141445 * 100.0
    : (Math.pow(yTxt, 0.62) - Math.pow(yBg, 0.57)) * 1.141445 * 100.0;

  const absLc = Math.abs(lc);
  return {
    lc,
    is_dark_on_light: isDarkOnLight,
    body_text: absLc >= 75.0,
    fluent_text: absLc >= 60.0,
    large_text: absLc >= 45.0,
    spot_text: absLc >= 30.0,
    non_text: absLc >= 15.0,
  };
}

export function auditContrastHex(fgHex: string, bgHex: string): ContrastAuditResult | null {
  const fg = parseHex(fgHex);
  const bg = parseHex(bgHex);
  if (!fg || !bg) return null;

  return {
    wcag: auditWcag21(fg, bg),
    apca: auditApca(fg, bg),
  };
}

export function generateHarmony(baseHex: string, mode: HarmonyMode, count = 5): SwatchItem[] {
  const baseOk = hexToOklch(baseHex);
  if (!baseOk) return [];

  if (mode === 'monochromatic') {
    const offsets = count === 5 ? [0.20, 0.10, 0.0, -0.10, -0.20] : [0.15, 0.0, -0.15];
    return offsets.map((dl) => {
      const l = Math.max(0.08, Math.min(0.94, baseOk.l + dl));
      return {
        hex: oklchToHex(l, baseOk.c, baseOk.h),
        l,
        c: baseOk.c,
        h: baseOk.h,
      };
    });
  }

  const offsetMap: Record<HarmonyMode, number[]> = {
    analogous: [-30.0, -15.0, 0.0, 15.0, 30.0],
    complementary: [0.0, 180.0],
    'split-complementary': [0.0, 150.0, 210.0],
    triadic: [0.0, 120.0, 240.0],
    tetradic: [0.0, 60.0, 180.0, 240.0],
    square: [0.0, 90.0, 180.0, 270.0],
    custom: [0.0, 72.0, 144.0, 216.0, 288.0],
    monochromatic: [],
  };

  const offsets = offsetMap[mode] || [0.0];
  const out: SwatchItem[] = [];

  for (let i = 0; i < count; i++) {
    const off = offsets[i % offsets.length];
    const cycle = Math.floor(i / offsets.length);
    const h = (baseOk.h + off + 360.0) % 360.0;
    const l = Math.max(0.08, Math.min(0.94, baseOk.l - cycle * 0.10));
    out.push({
      hex: oklchToHex(l, baseOk.c, h),
      l,
      c: baseOk.c,
      h,
    });
  }

  return out;
}

export function generateDesignTokens(seedHex: string): DesignSystemTokens | null {
  const baseOk = hexToOklch(seedHex);
  if (!baseOk) return null;

  const brand = seedHex.toUpperCase();
  const brandHover = oklchToHex(Math.max(0.08, baseOk.l - 0.08), baseOk.c, baseOk.h);
  const deep = oklchToHex(Math.max(0.08, baseOk.l - 0.18), baseOk.c * 0.9, baseOk.h);
  const bgLight = oklchToHex(0.98, Math.min(0.03, baseOk.c * 0.1), baseOk.h);
  const surfaceLight = oklchToHex(0.94, Math.min(0.04, baseOk.c * 0.1), baseOk.h);
  const inkLight = oklchToHex(0.15, Math.min(0.05, baseOk.c * 0.2), baseOk.h);
  const mutedLight = oklchToHex(0.45, Math.min(0.04, baseOk.c * 0.15), baseOk.h);
  const lineLight = oklchToHex(0.88, Math.min(0.03, baseOk.c * 0.1), baseOk.h);

  const brandDark = oklchToHex(Math.min(0.85, Math.max(0.45, baseOk.l + 0.12)), baseOk.c, baseOk.h);
  const brandDarkHover = oklchToHex(Math.min(0.90, Math.max(0.50, baseOk.l + 0.18)), baseOk.c, baseOk.h);
  const bgDark = oklchToHex(0.08, Math.min(0.04, baseOk.c * 0.15), baseOk.h);
  const surfaceDark = oklchToHex(0.14, Math.min(0.04, baseOk.c * 0.15), baseOk.h);
  const surface2Dark = oklchToHex(0.19, Math.min(0.04, baseOk.c * 0.15), baseOk.h);
  const inkDark = oklchToHex(0.94, Math.min(0.03, baseOk.c * 0.1), baseOk.h);
  const mutedDark = oklchToHex(0.68, Math.min(0.04, baseOk.c * 0.12), baseOk.h);
  const lineDark = oklchToHex(0.24, Math.min(0.04, baseOk.c * 0.15), baseOk.h);

  return {
    light: {
      '--brand': brand,
      '--brand-hover': brandHover,
      '--deep': deep,
      '--bg': bgLight,
      '--surface': '#FFFFFF',
      '--surface-2': surfaceLight,
      '--ink': inkLight,
      '--muted': mutedLight,
      '--line': lineLight,
    },
    dark: {
      '--brand': brandDark,
      '--brand-hover': brandDarkHover,
      '--bg': bgDark,
      '--surface': surfaceDark,
      '--surface-2': surface2Dark,
      '--ink': inkDark,
      '--muted': mutedDark,
      '--line': lineDark,
    },
  };
}

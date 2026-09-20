export interface RgbColor {
  r: number;
  g: number;
  b: number;
  a?: number;
}

export interface OklchColor {
  l: number;
  c: number;
  h: number;
  alpha?: number;
}

export interface HslColor {
  h: number;
  s: number;
  l: number;
  a?: number;
}

export interface HsvColor {
  h: number;
  s: number;
  v: number;
  a?: number;
}

export interface CielabColor {
  l: number;
  a: number;
  b: number;
  alpha?: number;
}

export interface WcagVerdict {
  ratio: number;
  normal_aa: boolean;
  large_aa: boolean;
  normal_aaa: boolean;
  large_aaa: boolean;
}

export interface ApcaVerdict {
  lc: number;
  is_dark_on_light: boolean;
  body_text: boolean;
  fluent_text: boolean;
  large_text: boolean;
  spot_text: boolean;
  non_text: boolean;
}

export interface ContrastAuditResult {
  wcag: WcagVerdict;
  apca: ApcaVerdict;
}

export type HarmonyMode =
  | 'analogous'
  | 'complementary'
  | 'split-complementary'
  | 'triadic'
  | 'tetradic'
  | 'square'
  | 'monochromatic'
  | 'custom';

export type CvdType = 'protanopia' | 'deuteranopia' | 'tritanopia' | 'achromatopsia';

export interface CvdAuditResult {
  cvd_type: CvdType;
  simulated_fg: string;
  simulated_bg: string;
  wcag: WcagVerdict;
  apca: ApcaVerdict;
}

export interface SwatchItem {
  hex: string;
  l: number;
  c: number;
  h: number;
}

export interface DesignSystemTokens {
  light: Record<string, string>;
  dark: Record<string, string>;
}

export type TargetGamut = 'srgb' | 'p3' | 'display-p3' | 'rec2020';

export interface GamutCusp {
  hue: number;
  lightness: number;
  max_chroma: number;
}

export interface GamutMappingResult {
  original: OklchColor;
  mapped: OklchColor;
  in_gamut: boolean;
  iterations: number;
  target_gamut: TargetGamut;
}

export interface QuantizedColor {
  rgb: RgbColor;
  oklch: OklchColor;
  pixel_count: number;
  weight: number;
}



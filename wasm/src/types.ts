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

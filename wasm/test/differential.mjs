// The same colour science exists twice in this repository: in Rust, under
// core/, and in TypeScript, under wasm/src/. The TypeScript port is what the
// browser runs when the WebAssembly module is not loaded, so the two are only
// interchangeable while they agree.
//
// This compares them directly, function by function, over the same inputs.
// It imports the compiled TypeScript engine and the Rust engine built for
// Node, and never goes through initColorustWasm, so each side is unambiguous.
//
//   npm run build            (tsc, writes dist/)
//   npm run build:wasm:node  (wasm-pack, writes pkg-node/)
//   npm test

import { test } from "node:test";
import assert from "node:assert/strict";

import * as ts from "../dist/index.js";
import * as rs from "../pkg-node/colorust_wasm.js";

const EPSILON = 1e-6;

const HEXES = [
  "#000000", "#FFFFFF", "#3B82F6", "#F7931E", "#0A2540",
  "#10B981", "#EF4444", "#7C3AED", "#64748B", "#FFD166",
];

const MODES = [
  "analogous", "complementary", "split-complementary", "triadic",
  "tetradic", "square", "monochromatic", "custom",
];

const CVDS = ["protanopia", "deuteranopia", "tritanopia", "achromatopsia"];
const GAMUTS = ["srgb", "p3", "rec2020"];

/** Compares two values of any shape, allowing floats to differ by EPSILON. */
function agree(a, b, path = "") {
  if (typeof a === "number" && typeof b === "number") {
    const delta = Math.abs(a - b);
    assert.ok(
      delta <= EPSILON,
      `${path}: ${a} against ${b}, off by ${delta}`,
    );
    return;
  }
  if (Array.isArray(a) || Array.isArray(b)) {
    assert.ok(Array.isArray(a) && Array.isArray(b), `${path}: one side is not an array`);
    assert.equal(a.length, b.length, `${path}: lengths ${a.length} and ${b.length}`);
    a.forEach((item, i) => agree(item, b[i], `${path}[${i}]`));
    return;
  }
  if (a && b && typeof a === "object" && typeof b === "object") {
    const keys = new Set([...Object.keys(a), ...Object.keys(b)]);
    for (const key of keys) {
      assert.ok(key in a, `${path}.${key}: missing on the TypeScript side`);
      assert.ok(key in b, `${path}.${key}: missing on the Rust side`);
      agree(a[key], b[key], `${path}.${key}`);
    }
    return;
  }
  assert.equal(a, b, `${path}: ${a} against ${b}`);
}

test("hex to OKLCH", () => {
  for (const hex of HEXES) {
    agree(ts.hexToOklch(hex), rs.hex_to_oklch_wasm(hex), hex);
  }
});

test("OKLCH to hex", () => {
  for (const hex of HEXES) {
    const c = ts.hexToOklch(hex);
    agree(
      ts.oklchToHex(c.l, c.c, c.h),
      rs.oklch_to_hex_wasm(c.l, c.c, c.h),
      hex,
    );
  }
});

test("contrast audit, WCAG 2.1 and APCA", () => {
  for (const fg of HEXES) {
    for (const bg of HEXES) {
      agree(
        ts.auditContrastHex(fg, bg),
        rs.audit_contrast_wasm(fg, bg),
        `${fg} on ${bg}`,
      );
    }
  }
});

test("harmonies, every mode", () => {
  for (const hex of HEXES) {
    for (const mode of MODES) {
      agree(
        ts.generateHarmony(hex, mode, 5),
        rs.generate_harmony_wasm(hex, mode, 5),
        `${hex} ${mode}`,
      );
    }
  }
});

test("design system tokens", () => {
  for (const hex of HEXES) {
    agree(ts.generateDesignTokens(hex), rs.generate_tokens_wasm(hex), hex);
  }
});

test("colour vision deficiency simulation", () => {
  for (const hex of HEXES) {
    for (const cvd of CVDS) {
      for (const severity of [0.5, 1.0]) {
        const rgb = ts.parseHex(hex);
        agree(
          ts.rgbToHex(ts.simulateCvd(rgb, cvd, severity)),
          rs.simulate_cvd_wasm(hex, cvd, severity),
          `${hex} ${cvd} at ${severity}`,
        );
      }
    }
  }
});

test("contrast under simulated deficiency", () => {
  for (const cvd of CVDS) {
    agree(
      ts.auditCvdContrast("#FFFFFF", "#0A2540", cvd, 1.0),
      rs.audit_cvd_contrast_wasm("#FFFFFF", "#0A2540", cvd, 1.0),
      cvd,
    );
  }
});

test("gamut membership", () => {
  for (const hex of HEXES) {
    const c = ts.hexToOklch(hex);
    for (const gamut of GAMUTS) {
      agree(
        ts.isInGamut(c, gamut),
        rs.is_in_gamut_wasm(c.l, c.c, c.h, gamut),
        `${hex} in ${gamut}`,
      );
    }
  }
});

test("gamut mapping", () => {
  // Chroma well past what sRGB holds, so the search actually runs.
  for (const hue of [0, 60, 140, 220, 300]) {
    for (const gamut of GAMUTS) {
      agree(
        ts.mapToGamut({ l: 0.7, c: 0.4, h: hue, alpha: 1.0 }, gamut, 1e-4),
        rs.map_to_gamut_wasm(0.7, 0.4, hue, gamut, 1e-4),
        `hue ${hue} into ${gamut}`,
      );
    }
  }
});

test("gamut cusp", () => {
  for (const hue of [0, 60, 140, 220, 300]) {
    for (const gamut of GAMUTS) {
      agree(
        ts.findGamutCusp(hue, gamut),
        rs.find_gamut_cusp_wasm(hue, gamut),
        `hue ${hue} in ${gamut}`,
      );
    }
  }
});

test("image quantisation", () => {
  // Three blocks of one colour each, in shares of 60, 30 and 10 percent.
  const pixels = [];
  const push = (n, r, g, b) => {
    for (let i = 0; i < n; i++) pixels.push(r, g, b, 255);
  };
  push(60, 59, 130, 246);
  push(30, 247, 147, 30);
  push(10, 16, 185, 129);

  const bytes = new Uint8Array(pixels);
  agree(ts.quantizeImage(bytes, 3, 20), rs.quantize_image_wasm(bytes, 3, 20), "three blocks");
});

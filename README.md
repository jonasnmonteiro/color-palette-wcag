# Color Palette & Accessibility Contrast Auditor (AuraColor Suite)

An ultra-high-performance color science suite, perceptual palette generator, and accessibility contrast auditor built in **Rust (WebAssembly)**, **TypeScript**, **Astro**, and **Bun/Playwright**.

---

## Architecture Overview

```
┌────────────────────────────────────────────────────────────────────────────┐
│                    AURACOLOR SYSTEM ARCHITECTURE                           │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│   ┌────────────────────────┐                    ┌───────────────────────┐  │
│   │   Astro Web Studio     │                    │  Bun Headless Crawler │  │
│   │   (Interactive Island) │                    │  (A11y Audit Engine)  │  │
│   └───────────┬────────────┘                    └───────────┬───────────┘  │
│               │                                             │              │
│               ▼ (Wasm / 120 FPS)                            ▼ (IPC / CLI)  │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │                     Rust High-Performance Core                      │  │
│   │  • OKLCH / Oklab / CIELAB / sRGB / HSV Matrix Color Spaces          │  │
│   │  • WCAG 2.1 (Relative Luminance) & WCAG 3.0 APCA Contrast ($L^c$)   │  │
│   │  • Delta E 2000 ($\Delta E_{00}$) Perceptual Color Difference       │  │
│   │  • 8 Harmonic Projections & Gamut Clipping / Mapping Engine         │  │
│   │  • Multi-Target Token Exporter (Figma, Tailwind, CSS, Style Dict)   │  │
│   └─────────────────────────────────────────────────────────────────────┘  │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

---

## Features

- **Perceptually Uniform Color Spaces:**
  - Full native support for **OKLCH**, **Oklab**, **CIELAB**, **sRGB Linear**, and **HSV**.
  - Eliminates the perceptual brightness anomalies inherent in standard HSL color wheels.
- **Dual Accessibility Contrast Standards:**
  - **W3C WCAG 2.1:** Standard relative luminance formula with AA ($4.5:1$), Large AA ($3.0:1$), and AAA ($7.0:1$) gates.
  - **W3C WCAG 3.0 APCA (Advanced Perceptual Contrast Algorithm):** Modern lightness contrast ($L^c$) calculations weighted by spatial frequency, text weight, and polarity.
- **8 Geometric Harmonic Modes:**
  - Analogous, Complementary, Split Complementary, Triadic, Tetradic, Square, Monochromatic (Luminance Ramp), and Custom Polygons.
- **Zero-Upload Dominant Image Color Quantization:**
  - Client-side image extraction using high-speed 2D canvas bucketing.
- **Headless URL Accessibility Crawler:**
  - Automated crawling engine powered by Bun and Playwright extracting computed DOM styles and auditing live websites for accessibility compliance (ADA / European Accessibility Act).
- **Multi-Format Design Token Exporter:**
  - Direct export to **Figma Tokens JSON (Tokens Studio / W3C Community Group)**, **Tailwind CSS v3 & v4**, **CSS Custom Properties**, **SwiftUI Color Sets**, and **Android XML/Compose**.

---

## Repository Structure

```text
color-palette-wcag/
├── core/                  # Rust 2021 Core Math & Color Science Engine
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── spaces.rs      # sRGB, Linear, Oklab, OKLCH, CIELAB conversions
│       ├── contrast.rs    # WCAG 2.1 & WCAG 3.0 APCA algorithms
│       ├── harmonies.rs   # 8 Geometric harmony projection kernels
│       ├── delta_e.rs     # CIEDE2000 color distance metric
│       └── exporter.rs    # Design token serializer
├── wasm/                  # WebAssembly bridge via wasm-bindgen
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── crawler/               # Bun + Playwright Headless URL A11y Auditor
│   ├── package.json
│   └── src/
│       └── auditor.ts
└── web/                   # Astro web application & interactive Studio Island
    ├── astro.config.mjs
    ├── package.json
    └── src/
        ├── components/
        │   └── ColorStudio.tsx
        ├── pages/
        │   ├── index.astro
        │   ├── auditor.astro
        │   └── docs.astro
        └── styles/
```

---

## License

This project is open-source under the [MIT License](LICENSE).

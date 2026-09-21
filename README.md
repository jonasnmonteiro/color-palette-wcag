# Color Palette & Accessibility Contrast Auditor (Colorust Suite)

A perceptual color science engine, palette generator, and accessibility contrast
auditor. The color science is written in **Rust**, with a matching **TypeScript**
port for the browser, an **Astro** studio on top of it, and a **Bun/Playwright**
crawler that audits live pages.

---

## Architecture Overview

```
┌────────────────────────────────────────────────────────────────────────────┐
│                    COLORUST SYSTEM ARCHITECTURE                            │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│   ┌────────────────────────┐                    ┌───────────────────────┐  │
│   │   Astro Web Studio     │                    │  Bun Headless Crawler │  │
│   │   (Interactive Island) │                    │  (A11y Audit Engine)  │  │
│   └───────────┬────────────┘                    └───────────┬───────────┘  │
│               │                                             │              │
│               ▼ (TypeScript engine)                         ▼ (IPC / CLI)  │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │                     Color Science Core                              │  │
│   │  • OKLCH / Oklab / CIELAB / sRGB / HSL / HSV conversions            │  │
│   │  • WCAG 2.1 (relative luminance) and WCAG 3.0 APCA (Lc)             │  │
│   │  • Delta E 2000 perceptual color difference                         │  │
│   │  • Color vision deficiency simulation                               │  │
│   │  • 8 harmonic projections, gamut clipping and mapping               │  │
│   │  • Multi-target token exporter (Figma, Tailwind, CSS, Swift, XML)   │  │
│   └─────────────────────────────────────────────────────────────────────┘  │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

**Where each engine runs.** The Rust crate in `core/` is the reference
implementation: it holds the tests, the benchmarks and the native CLI.
`wasm/src/lib.rs` compiles it to WebAssembly through `wasm-bindgen`, and the
studio loads that build when it is served beside the page. The TypeScript port
in `wasm/src/index.ts` implements the same algorithms and runs when the
WebAssembly module is not there, which is what happens on a machine with no
Rust toolchain.

Two implementations are only interchangeable while they agree, so
`wasm/test/differential.mjs` runs both over the same inputs and compares every
field of every result. It is the reason the fallback is a design and not a
hope.

---

## Quick start

### The CLI

Needs a Rust toolchain. Five subcommands, all taking hex colors. Every command
below runs from the `core/` directory.

```bash
cd core
cargo run -- audit "#FFFFFF" "#0A2540"
```
Reports WCAG 2.1 ratios against the AA, large AA and AAA gates, and the APCA Lc
value against its five thresholds.

```bash
cargo run -- harmony "#3B82F6" --mode triadic --count 5
```
Modes: `analogous`, `complementary`, `split-complementary`, `triadic`,
`tetradic`, `square`, `monochromatic`, `custom`.

```bash
cargo run -- tokens "#3B82F6" --format css
```
Formats: `css`, `scss`, `tailwind-v3`, `tailwind-v4`, `figma-tokens`,
`style-dictionary`, `swift`, `android-xml`.

```bash
cargo run -- gamut "#00FF88" --target p3
```
Targets: `srgb`, `p3`, `rec2020`. Reports whether the color fits and, when it
does not, the mapped result and the number of binary search steps it took.

```bash
cargo run -- convert "#3B82F6"
```
Prints the same color in every supported space.

Build a release binary with `cargo build --release`; it lands at
`core/target/release/colorust`.

### The web studio

```bash
cd web
npm install
npm run dev
```

Three pages: the studio at `/`, the contrast auditor at `/auditor/`, and the
token exporter at `/tokens/`. `npm run build` emits a static site into
`web/dist/`, and it can be served from a subpath by setting `base` in the Astro
config.

To run the Rust engine in the browser, build it first:

```bash
cd wasm && npm run build:wasm
```

`web`'s own build copies the result into `public/wasm/` and the studio loads it
from there. Without it the studio uses the TypeScript engine and says so in the
build output.

Both themes are in `src/styles/global.css`: the dark one on `:root` and the
light one on `[data-theme="light"]`. `public/theme-boot.js` applies the stored
choice before the first paint, and the default is whatever `data-theme` the
document already carries, so a deployment can ship a different one without
touching the script.

### The crawler

Needs [Bun](https://bun.sh) and a Playwright chromium.

```bash
cd crawler
bun install
bun run src/cli.ts https://example.com --format markdown
```

It walks the rendered DOM, resolves each node's **effective** background by
compositing semi-transparent ancestors, reads `font-size` and `font-weight` to
classify large text, and reports WCAG 2.1 and APCA per node. Output is JSON or
Markdown.

### Tests and benchmarks

```bash
cd core
cargo test          # 29 integration tests
cargo bench         # Criterion benchmarks
```

The two engines are compared against each other from `wasm/`:

```bash
cd wasm
npm run build            # the TypeScript engine, into dist/
npm run build:wasm:node  # the Rust engine, into pkg-node/
npm test                 # every function, both engines, same inputs
```

---

## Implemented

- **Perceptually uniform color spaces.** OKLCH, Oklab, CIELAB, linear sRGB, HSL
  and HSV, with conversions in both directions.
- **Two contrast standards.** WCAG 2.1 relative luminance with the AA (4.5:1),
  large AA (3.0:1) and AAA (7.0:1) gates, and WCAG 3.0 APCA lightness contrast
  with the polarity-aware exponents and the five Lc thresholds.
- **Delta E 2000** perceptual color difference.
- **Color vision deficiency simulation** for protanopia, deuteranopia,
  tritanopia and achromatopsia, with a severity parameter, plus contrast audits
  under simulated deficiency.
- **8 harmonic modes:** analogous, complementary, split complementary, triadic,
  tetradic, square, monochromatic and custom polygons.
- **Gamut handling** for sRGB, Display P3 and Rec. 2020, including cusp finding
  and binary search gamut mapping.
- **Dominant color extraction** by k-means clustering in Oklab rather than in
  RGB, so the clusters follow perceived difference. Runs in the page, and no
  image is uploaded anywhere.
- **Headless accessibility crawler** over live URLs.
- **Design token export** to CSS custom properties, SCSS, Tailwind v3 and v4,
  Figma Tokens JSON, Style Dictionary, SwiftUI and Android XML.

---

## Repository structure

```text
colorust/
├── core/                    Rust color science engine, CLI, tests, benchmarks
│   ├── src/
│   │   ├── spaces.rs        sRGB, linear, Oklab, OKLCH, CIELAB, HSL, HSV
│   │   ├── contrast.rs      WCAG 2.1 and APCA
│   │   ├── delta_e.rs       CIEDE2000
│   │   ├── cvd.rs           color vision deficiency simulation
│   │   ├── gamut.rs         gamut tests, cusp finding, mapping
│   │   ├── harmonies.rs     the 8 harmonic modes
│   │   ├── palette.rs       swatches and design system tokens
│   │   ├── quantization.rs  k-means in Oklab
│   │   ├── exporter.rs      the 8 token formats
│   │   └── bin/colorust.rs  the CLI
│   ├── tests/               29 integration tests
│   └── benches/             Criterion benchmarks
├── wasm/                    WebAssembly bindings and the TypeScript port
│   ├── src/lib.rs           wasm-bindgen exports
│   ├── src/index.ts         the same algorithms in TypeScript
│   ├── src/exporter.ts      token export in TypeScript
│   └── build.sh             wasm-pack build
├── crawler/                 Bun and Playwright accessibility auditor
│   └── src/auditor.ts       effective background resolution and per-node audit
└── web/                     Astro studio
    ├── src/components/ColorStudioIsland.astro
    ├── src/pages/           index, auditor, tokens
    ├── src/styles/global.css
    └── tools/leaked-colors.py   fails the build on a raw color outside the tokens
```

---

## License

MIT. See [LICENSE](LICENSE).

// Copies the wasm-pack output into public/, where the site serves it as a
// static file and the engine loads it at runtime.
//
// The studio works without it: wasm/src/index.ts falls back to its TypeScript
// implementations when the module is not there, which is what happens on a
// machine with no Rust toolchain. So a missing pkg/ is reported and not an
// error.

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const from = path.resolve(here, "../../wasm/pkg");
const to = path.resolve(here, "../public/wasm");

const NEEDED = ["colorust_wasm.js", "colorust_wasm_bg.wasm"];

if (!fs.existsSync(from)) {
  console.log("copy-wasm: wasm/pkg is not built, the studio will use the TypeScript engine");
  console.log("copy-wasm: build it with  cd ../wasm && npm run build:wasm");
  process.exit(0);
}

const missing = NEEDED.filter((name) => !fs.existsSync(path.join(from, name)));
if (missing.length) {
  console.log(`copy-wasm: wasm/pkg is incomplete (${missing.join(", ")}), using the TypeScript engine`);
  process.exit(0);
}

fs.rmSync(to, { recursive: true, force: true });
fs.mkdirSync(to, { recursive: true });
for (const name of NEEDED) {
  fs.copyFileSync(path.join(from, name), path.join(to, name));
}

const bytes = fs.statSync(path.join(to, "colorust_wasm_bg.wasm")).size;
console.log(`copy-wasm: ${NEEDED.length} files into public/wasm, ${(bytes / 1024).toFixed(0)} KB of WebAssembly`);

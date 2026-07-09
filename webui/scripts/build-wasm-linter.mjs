import { copyFileSync, existsSync, mkdirSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

function run(command, args, cwd, env = process.env) {
  const result = spawnSync(command, args, {
    cwd,
    env,
    stdio: "inherit",
    shell: process.platform === "win32"
  });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(" ")} failed with exit code ${result.status ?? -1}`);
  }
}

function ensureFile(path, label) {
  if (!existsSync(path)) {
    throw new Error(`${label} not found: ${path}`);
  }
}

function copyFileTo(from, to, label) {
  mkdirSync(dirname(to), { recursive: true });
  copyFileSync(from, to);
  console.log(`${label}: ${to}`);
}

const thisFile = fileURLToPath(import.meta.url);
const scriptsDir = dirname(thisFile);
const repoRoot = resolve(scriptsDir, "..");
const rustscriptRoot = resolve(process.env.RUSTSCRIPT_REPO ?? resolve(repoRoot, "..", "..", "rustscript"));
const wasmTarget = "wasm32-unknown-unknown";
const wasmName = "pd_vm_wasm.wasm";
const cargoTargetDir = resolve(rustscriptRoot, "target");
const compiledWasmPath = resolve(cargoTargetDir, wasmTarget, "release", wasmName);
const rssGrammarSrc = resolve(rustscriptRoot, "editor-assets", "monaco", "rss.tmLanguage.json");
const rssConfigSrc = resolve(rustscriptRoot, "editor-assets", "monaco", "rss.language-configuration.json");
const grammarOutDir = resolve(repoRoot, "src", "app", "monaco");

ensureFile(resolve(rustscriptRoot, "Cargo.toml"), "RustScript repository");
run("rustup", ["target", "add", wasmTarget], rustscriptRoot);
run("cargo", ["build", "-p", "pd-vm-wasm", "--target", wasmTarget, "--release"], rustscriptRoot, {
  ...process.env,
  CARGO_TARGET_DIR: cargoTargetDir
});
ensureFile(compiledWasmPath, "compiled linter wasm");
copyFileTo(compiledWasmPath, resolve(repoRoot, "public", "wasm", wasmName), "copied controller linter wasm");
copyFileTo(rssGrammarSrc, resolve(grammarOutDir, "rss.tmLanguage.json"), "synced RustScript grammar");
copyFileTo(rssConfigSrc, resolve(grammarOutDir, "rss.language-configuration.json"), "synced RustScript language config");

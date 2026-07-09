import { copyFileSync, existsSync, mkdirSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const ALL_TARGETS = ["controller"];

const thisFile = fileURLToPath(import.meta.url);
const scriptsDir = dirname(thisFile);
const repoRoot = resolve(scriptsDir, "..");
const rustscriptRoot = resolve(repoRoot, "..", "rustscript");

const wasmTarget = "wasm32-unknown-unknown";
const wasmName = "pd_vm_wasm.wasm";
const cargoTargetDir = resolve(rustscriptRoot, "target");
const compiledWasmPath = resolve(cargoTargetDir, wasmTarget, "release", wasmName);

const rssGrammarSrc = resolve(rustscriptRoot, "pd-vm", "webui", "src", "monaco", "rss.tmLanguage.json");
const rssConfigSrc = resolve(rustscriptRoot, "pd-vm", "webui", "src", "monaco", "rss.language-configuration.json");

const targetSpecs = {
  controller: {
    needsRuntime: false,
    wasmOut: resolve(repoRoot, "pd-controller", "webui", "public", "wasm", wasmName),
    grammarOutDir: resolve(repoRoot, "pd-controller", "webui", "src", "app", "monaco")
  }
};

function run(command, args, cwd = repoRoot, env = process.env) {
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

function normalizeTargets(rawTargets) {
  if (rawTargets.length === 0 || rawTargets.includes("all")) {
    return ALL_TARGETS;
  }

  const uniqueTargets = [...new Set(rawTargets)];
  for (const target of uniqueTargets) {
    if (!ALL_TARGETS.includes(target)) {
      throw new Error(
        `unknown sync target '${target}'. Expected one of: ${[...ALL_TARGETS, "all"].join(", ")}`
      );
    }
  }
  return uniqueTargets;
}

function ensureFileExists(path, label) {
  if (!existsSync(path)) {
    throw new Error(`${label} not found: ${path}`);
  }
}

function copyFileTo(pathFrom, pathTo, label) {
  mkdirSync(dirname(pathTo), { recursive: true });
  copyFileSync(pathFrom, pathTo);
  console.log(`${label}: ${pathTo}`);
}

function copyGrammarAssets(grammarOutDir) {
  ensureFileExists(rssGrammarSrc, "RustScript grammar");
  ensureFileExists(rssConfigSrc, "RustScript language configuration");
  mkdirSync(grammarOutDir, { recursive: true });
  copyFileTo(rssGrammarSrc, resolve(grammarOutDir, "rss.tmLanguage.json"), "synced RustScript grammar");
  copyFileTo(
    rssConfigSrc,
    resolve(grammarOutDir, "rss.language-configuration.json"),
    "synced RustScript language config"
  );
}

function buildWasm({ runtime }) {
  ensureFileExists(resolve(rustscriptRoot, "Cargo.toml"), "rustscript workspace");
  const args = ["build", "-p", "pd-vm-wasm"];
  if (runtime) {
    args.push("--features", "runtime");
  }
  args.push("--target", wasmTarget, "--release");
  run("cargo", args, rustscriptRoot, {
    ...process.env,
    CARGO_TARGET_DIR: cargoTargetDir
  });
  ensureFileExists(compiledWasmPath, "compiled editor wasm");
}

function syncTargets(targets, runtime) {
  const targetsForBuild = targets.filter((target) => targetSpecs[target].needsRuntime === runtime);
  if (targetsForBuild.length === 0) {
    return;
  }

  buildWasm({ runtime });
  for (const target of targetsForBuild) {
    const spec = targetSpecs[target];
    copyFileTo(compiledWasmPath, spec.wasmOut, `copied ${target} wasm`);
    if (spec.grammarOutDir) {
      copyGrammarAssets(spec.grammarOutDir);
    }
  }
}

const targets = normalizeTargets(process.argv.slice(2));
run("rustup", ["target", "add", wasmTarget]);
syncTargets(targets, false);
syncTargets(targets, true);

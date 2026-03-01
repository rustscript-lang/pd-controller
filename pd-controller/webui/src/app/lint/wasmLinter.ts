import type { SourceFlavor } from "@/app/types";

export type LintSpan = {
  startLine: number;
  startColumn: number;
  endLine: number;
  endColumn: number;
};

export type LintDiagnostic = {
  line: number;
  message: string;
  span: LintSpan | null;
  rendered: string;
};

export type LintReport = {
  diagnostics: LintDiagnostic[];
};

type WasmLinterExports = {
  memory: WebAssembly.Memory;
  wasm_alloc(len: number): number;
  wasm_dealloc(ptr: number, len: number): void;
  lint_source_json(sourcePtr: number, sourceLen: number, flavorPtr: number, flavorLen: number): bigint;
};

const encoder = new TextEncoder();
const decoder = new TextDecoder("utf-8");
let wasmPromise: Promise<WasmLinterExports> | null = null;

function wasmPath(): string {
  const base = import.meta.env.BASE_URL ?? "/";
  return `${base.replace(/\/+$/, "/")}wasm/pd_vm_lint_wasm.wasm`;
}

function writeBytes(wasm: WasmLinterExports, bytes: Uint8Array): number {
  const ptr = wasm.wasm_alloc(bytes.length);
  const memory = new Uint8Array(wasm.memory.buffer);
  memory.set(bytes, ptr);
  return ptr;
}

function readBytes(wasm: WasmLinterExports, ptr: number, len: number): Uint8Array {
  return new Uint8Array(wasm.memory.buffer, ptr, len);
}

function unpackPtrLen(packed: bigint): { ptr: number; len: number } {
  const ptr = Number(packed & 0xFFFF_FFFFn);
  const len = Number((packed >> 32n) & 0xFFFF_FFFFn);
  return { ptr, len };
}

function normalizeReport(parsed: unknown): LintReport {
  if (!parsed || typeof parsed !== "object" || !("diagnostics" in parsed)) {
    return { diagnostics: [] };
  }
  const diagnosticsRaw = (parsed as { diagnostics?: unknown }).diagnostics;
  if (!Array.isArray(diagnosticsRaw)) {
    return { diagnostics: [] };
  }
  const diagnostics: LintDiagnostic[] = [];
  for (const item of diagnosticsRaw) {
    if (!item || typeof item !== "object") {
      continue;
    }
    const lineRaw = Number((item as { line?: unknown }).line);
    const messageRaw = (item as { message?: unknown }).message;
    const renderedRaw = (item as { rendered?: unknown }).rendered;
    let span: LintSpan | null = null;
    const spanRaw = (item as { span?: unknown }).span;
    if (spanRaw && typeof spanRaw === "object") {
      const startLine = Number((spanRaw as { start_line?: unknown }).start_line);
      const startCol = Number((spanRaw as { start_col?: unknown }).start_col);
      const endLine = Number((spanRaw as { end_line?: unknown }).end_line);
      const endCol = Number((spanRaw as { end_col?: unknown }).end_col);
      if (
        Number.isFinite(startLine) &&
        Number.isFinite(startCol) &&
        Number.isFinite(endLine) &&
        Number.isFinite(endCol)
      ) {
        span = {
          startLine: Math.max(1, Math.trunc(startLine)),
          startColumn: Math.max(1, Math.trunc(startCol)),
          endLine: Math.max(1, Math.trunc(endLine)),
          endColumn: Math.max(1, Math.trunc(endCol))
        };
      }
    }
    const line = Number.isFinite(lineRaw) ? Math.max(0, Math.trunc(lineRaw)) : 0;
    const message = typeof messageRaw === "string" ? messageRaw : "";
    const rendered = typeof renderedRaw === "string" ? renderedRaw : message;
    if (!message) {
      continue;
    }
    diagnostics.push({
      line,
      message,
      span,
      rendered
    });
  }
  return { diagnostics };
}

async function loadWasm(): Promise<WasmLinterExports> {
  if (!wasmPromise) {
    wasmPromise = (async () => {
      const response = await fetch(wasmPath());
      if (!response.ok) {
        throw new Error(`failed to fetch wasm linter (${response.status})`);
      }
      const bytes = await response.arrayBuffer();
      const { instance } = await WebAssembly.instantiate(bytes, {});
      const exports = instance.exports as Partial<WasmLinterExports>;
      if (
        !exports.memory ||
        typeof exports.wasm_alloc !== "function" ||
        typeof exports.wasm_dealloc !== "function" ||
        typeof exports.lint_source_json !== "function"
      ) {
        throw new Error("invalid wasm linter exports");
      }
      return exports as WasmLinterExports;
    })();
  }
  return wasmPromise;
}

export async function lintWithWasm(source: string, flavor: SourceFlavor): Promise<LintReport> {
  const wasm = await loadWasm();
  const sourceBytes = encoder.encode(source);
  const flavorBytes = encoder.encode(flavor);
  let sourcePtr = 0;
  let flavorPtr = 0;
  let resultPtr = 0;
  let resultLen = 0;

  try {
    sourcePtr = writeBytes(wasm, sourceBytes);
    flavorPtr = writeBytes(wasm, flavorBytes);
    const packed = wasm.lint_source_json(sourcePtr, sourceBytes.length, flavorPtr, flavorBytes.length);
    const unpacked = unpackPtrLen(packed);
    resultPtr = unpacked.ptr;
    resultLen = unpacked.len;
    if (resultPtr === 0 || resultLen === 0) {
      return { diagnostics: [] };
    }
    const json = decoder.decode(readBytes(wasm, resultPtr, resultLen));
    return normalizeReport(JSON.parse(json));
  } finally {
    if (sourcePtr !== 0) {
      wasm.wasm_dealloc(sourcePtr, sourceBytes.length);
    }
    if (flavorPtr !== 0) {
      wasm.wasm_dealloc(flavorPtr, flavorBytes.length);
    }
    if (resultPtr !== 0 && resultLen > 0) {
      wasm.wasm_dealloc(resultPtr, resultLen);
    }
  }
}

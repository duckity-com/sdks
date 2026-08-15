import * as Comlink from "comlink";
import init, { process, type InitOutput } from "@duckity/wasm";
import wasm from "@duckity/wasm/duckity_bg.wasm";

console.time("duckity.wasm.init");
const a: WebAssembly.Module = await wasm();
const b: InitOutput = await init(a);
console.timeEnd("duckity.wasm.init");

/**
 * Expose the `process` function from the WASM module to the main thread via Comlink.
 *
 * This allows the main thread to call the `process` function in the worker thread, which will
 * handle the processing of the challenges without blocking the main thread.
 */
const api = {
  async solve(challenge: string) {
    console.time("duckity.wasm.process");
    let solution = process(challenge);
    console.timeEnd("duckity.wasm.process");
    return solution;
  },
};

Comlink.expose(api);

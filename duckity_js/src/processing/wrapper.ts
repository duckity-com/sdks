import * as Comlink from "comlink";

/**
 * Web Worker that handles the processing of the challenges.
 */
const worker = new Worker(new URL("./worker.js", import.meta.url), {
  type: "module",
});

/**
 * Wrap the worker with Comlink to allow for easy communication between the main thread and the
 * worker thread. The worker exposes a `solve` function that takes a challenge string and returns a
 * promise that resolves to the solution string.
 */
export default Comlink.wrap<{
  solve(challenge: string): Promise<string>;
}>(worker);

import * as Comlink from "comlink";

type WorkerApi = {
  solve(challenge: string): Promise<string>;
};

let worker: Worker | undefined;
let api: Comlink.Remote<WorkerApi> | undefined;

/**
 * Wraps the worker with Comlink to allow for easy communication between the main thread and the
 * worker thread. The API exposes a `solve` function that takes a challenge string and returns a
 * promise that resolves to the solution string.
 */
export function getWorkerApi(): Comlink.Remote<WorkerApi> {
  if (typeof window === "undefined") {
    throw new Error("Worker API can only be used in the browser");
  }

  if (!api) {
    worker = new Worker(new URL("./worker.js", import.meta.url), {
      type: "module",
    });

    api = Comlink.wrap<WorkerApi>(worker);
  }

  return api;
}

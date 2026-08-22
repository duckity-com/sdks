import * as Comlink from "comlink";
import processing from ".";

/**
 * Solves a challenge in the background.
 */
const api = {
  async solve(challenge: string) {
    let decodedChallenge = processing.decode(challenge);
    let solution = await processing.solve(decodedChallenge);
    let solutionString = processing.encode(challenge, solution);

    return solutionString;
  },
};

Comlink.expose(api);

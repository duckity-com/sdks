import wrapper from "./processing/wrapper";
import { post } from "./requests";

/**
 * Options for getting a Duckity challenge from the API.
 */
export interface GetDuckityChallengeOptions {
  /**
   * The custom-context threat correlation keys to be sent with the request.
   */
  keys?: { [key: string]: string };

  /**
   * The base URL to the API endpoint. Defaults to `https://api.duckity.com/d1` if not provided.
   * 
   * The version must be specified in the URL path.
   *
   * Only update this when self-hosting a Duckling.
   */
  api?: string;

  // /**
  //  * When this is `false`, the client will assume it is allowed to get a challenge the first time
  //  * one is requested. This may not always be true (when the client has an active penalty or used
  //  * all of their allowed requests in another session), case in which the client may be penalized.
  //  *
  //  * When this is `true`, an extra request will be made to the Duckling API to synchronize the
  //  * client's state with the server's state. This extra request will not issue a challenge, rather
  //  * only synchronize the client's state with the server's state. If the client is penalized or has
  //  * to wait before being allowed to get a challenge, the client will be informed of this. It'll
  //  * avoid getting it unnecessarily penalized at the cost of an extra request.
  //  */
  // sync?: boolean;
}

/**
 * The response from the Duckling API when requesting a challenge.
 */
interface ChallengeResponse {
  /**
   * The encoded challenge string returned by the Duckling API.
   */
  challenge: string;
}

/**
 * Fetches, solves, and returns the solution to a Duckity challenge for the given protection
 * profile ID.
 *
 * @param protectionProfileId The ID of the protection profile to get the challenge for.
 * @param options Optional parameters for the challenge issuance.
 * @returns The solution to the challenge issued by Duckity.
 */
export async function solve(
  protectionProfileId: string,
  options?: GetDuckityChallengeOptions,
): Promise<string> {
  if (options?.api && options.api.endsWith("/")) {
    // Remove the trailing slash from the API URL if provided
    options.api = options.api.slice(0, -1);
  }

  let response: ChallengeResponse = await post(
    `${options?.api || "https://api.duckity.com/d1"}/challenges/${protectionProfileId}/issue`,
    {
      body: {
        keys: options?.keys || {},
      },
    },
  );

  let solution = await wrapper.solve(response.challenge);

  return solution;
}

import { getWorkerApi } from "./processing/wrapper";
import { post } from "./requests";

/**
 * Options for getting a Duckity challenge from the API.
 */
export interface GetDuckityChallengeOptions {
  /**
   * The base URL to the API endpoint. Defaults to `https://api.duckity.com/d1` if not provided.
   *
   * The version must be specified in the URL path.
   *
   * Only update this when self-hosting a Duckling.
   */
  api?: string;
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
 * Fetches, solves, and returns the solution to a Duckity challenge for the given policy ID.
 *
 * @param policyId The ID of the policy to get the challenge for.
 * @param options Optional parameters for the challenge issuance.
 *
 * @returns The solution to the challenge issued by Duckity.
 */
export async function solve(
  policyId: string,
  options?: GetDuckityChallengeOptions,
): Promise<string> {
  if (options?.api && options.api.endsWith("/")) {
    // Remove the trailing slash from the API URL if provided
    options.api = options.api.slice(0, -1);
  }

  let response: ChallengeResponse = await post(
    `${options?.api || "https://api.duckity.com/d1"}/challenges/${policyId}/issue`,
  );

  let solution = await getWorkerApi().solve(response.challenge);

  return solution;
}

/**
 * Options for validating a Duckity solution with the API.
 */
export interface ValidateDuckitySolutionOptions {
  /**
   * The base URL to the API endpoint. Defaults to `https://api.duckity.com/d1` if not provided.
   *
   * The version must be specified in the URL path.
   *
   * Only update this when self-hosting a Duckling.
   */
  api?: string;
}

/**
 * The response from the Duckling API when validating a solution.
 */
interface ValidationResponse {
  /**
   * Whether the submitted solution is valid.
   */
  is_valid: boolean;
}

/**
 * Validates a client-submitted solution token.
 *
 * @param solution The solution token to validate.
 * @param clientIp The IP of the client that submitted the solution token.
 * @param applicationSecret The policy's application secret.
 * @param policyId The ID of the policy for which this challenge was issued.
 * @param options Additional options to customize the behavior of this call.
 *
 * @returns Whether the solution was valid or not.
 */
export async function validate(
  solution: string,
  clientIp: string,
  applicationSecret: string,
  policyId: string,
  options?: ValidateDuckitySolutionOptions,
): Promise<boolean> {
  if (options?.api && options.api.endsWith("/")) {
    // Remove the trailing slash from the API URL if provided
    options.api = options.api.slice(0, -1);
  }

  let response: ValidationResponse = await post(
    `${options?.api || "https://api.duckity.com/d1"}/challenges/${policyId}/validate`,
    {
      headers: {
        Authorization: `Bearer ${applicationSecret}`,
      },
      body: {
        solution,
        ip: clientIp,
      },
    },
  );

  return response.is_valid;
}

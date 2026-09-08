/**
 * Options for getting a Duckity challenge from the API.
 */
interface GetDuckityChallengeOptions {
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
 * Fetches, solves, and returns the solution to a Duckity challenge for the given protection
 * profile ID.
 *
 * @param protectionProfileId The ID of the protection profile to get the challenge for.
 * @param options Optional parameters for the challenge issuance.
 *
 * @returns The solution to the challenge issued by Duckity.
 */
declare function solve(protectionProfileId: string, options?: GetDuckityChallengeOptions): Promise<string>;
/**
 * Options for validating a Duckity solution with the API.
 */
interface ValidateDuckitySolutionOptions {
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
 * Validates a client-submitted solution token.
 *
 * @param solution The solution token to validate.
 * @param clientIp The IP of the client that submitted the solution token.
 * @param applicationSecret The protection profile's application secret.
 * @param protectionProfileId The ID of the protection profile for which this challenge was issued.
 * @param options Additional options to customize the behavior of this call.
 *
 * @returns Whether the solution was valid or not.
 */
declare function validate(solution: string, clientIp: string, applicationSecret: string, protectionProfileId: string, options?: ValidateDuckitySolutionOptions): Promise<boolean>;

export { solve, validate };
export type { GetDuckityChallengeOptions, ValidateDuckitySolutionOptions };

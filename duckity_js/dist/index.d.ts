/**
 * Options for getting a Duckity challenge from the API.
 */
interface GetDuckityChallengeOptions {
    /**
     * The custom-context threat correlation keys to be sent with the request.
     */
    keys?: {
        [key: string]: string;
    };
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
 * @returns The solution to the challenge issued by Duckity.
 */
declare function solve(protectionProfileId: string, options?: GetDuckityChallengeOptions): Promise<string>;

export { solve };
export type { GetDuckityChallengeOptions };

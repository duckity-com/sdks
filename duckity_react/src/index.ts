import { useEffect, useRef, useState } from "react";
import useSWR from "swr";
import * as duckity from "@duckity/js";

/**
 * The result returned by the `useChallenge` hook, containing the solve function, current status,
 * solution, and boolean flags for loading, error, and idle states.
 */
export interface UseChallengeResult {
  /**
   * Discards the current challenge solution and solves a new challenge.
   *
   * Note that if the challenge is currently being fetched or solved, calling this function will
   * not stop the ongoing process and the solution still will be set once that solving the
   * challenge completes.
   */
  refresh: () => void;

  /**
   * Returns a promise that resolves when a challenge solution is available.
   *
   * If a solution is already available, the promise resolves immediately. If no solution is
   * available yet, the promise resolves once a solution is obtained.
   *
   * If an error occurs while fetching or solving the challenge, the promise returned by this
   * function will reject with the same error.
   *
   * @returns A promise that resolves to the current challenge solution string once it is available.
   */
  wait: () => Promise<string>;

  /**
   * The current status of the challenge-solving process, which can be one of "solving", "solved",
   * or "error".
   *
   * When this field is "error", `UseChallengeResult.error` will be set.
   */
  status: UseChallengeStatus;

  /**
   * The latest solved challenge solution string, or undefined if no solution has been obtained yet.
   */
  solution?: string;

  /**
   * An error if any occurred, or undefined.
   */
  error?: any;
}

/**
 * The possible status values for the challenge-solving process.
 */
export type UseChallengeStatus = "solving" | "solved" | "error";

interface Waiter {
  /**
   * Resolves the waiter with the provided solution string once it is available.
   *
   * @param value The solution string to resolve with.
   */
  resolve: (value: string) => void;

  /**
   * Rejects the promise associated with the waiter with the provided reason if an error occurs
   * during fetching or solving.
   *
   * @param reason The error reason to reject with.
   */
  reject: (reason?: any) => void;
}

/**
 * Hook to handle fetching and solving a challenge for a given application and policy.
 *
 * This hook starts solving a challenge on initiation, and will get a new one on every call to
 * `refresh()`. Use `wait()` to wait until a solution is ready.
 *
 * @param policyId The ID of the policy to use to get a challenge.
 * @param options Additional configuration parameters, like the API's base URL.
 * @returns An object containing the the current status, solution, and functions to wait for a
 * solution and to refresh the challenge.
 */
export function useChallenge(
  policyId: string,
  options?: duckity.GetDuckityChallengeOptions,
) {
  const waiters = useRef<Waiter[]>([]);

  const { data, error, mutate, isLoading, isValidating } = useSWR<string>(
    `duckity:${policyId}`,
    () => duckity.solve(policyId, options),
  );

  let status: UseChallengeStatus = "solving";
  if (!!error) {
    status = "error";
  } else if (isLoading || isValidating) {
    status = "solving";
  } else {
    status = "solved";
  }

  useEffect(() => {
    if (!!data) {
      for (const waiter of waiters.current) {
        waiter.resolve(data);
      }
      waiters.current = [];
    } else if (!!error) {
      for (const waiter of waiters.current) {
        waiter.reject(error);
      }
      waiters.current = [];
    }
  }, [data, error, isLoading, isValidating]);

  return {
    solution: !isLoading && !isValidating ? data : undefined,
    error,
    status,
    refresh() {
      mutate(undefined);
    },
    wait(): Promise<string> {
      if (data && !isLoading && !isValidating) {
        return Promise.resolve(data);
      } else {
        return new Promise((resolve, reject) => {
          waiters.current.push({
            resolve,
            reject,
          });
        });
      }
    },
  };
}

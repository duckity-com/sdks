/**
 * Error schema for API responses. This is used to extract error messages from failed requests.
 */
interface ErrorMessage {
  title: string;
  message: string;
}

/**
 * Wrapper function for making HTTP requests using the Fetch API. It handles JSON parsing and error
 * handling, throwing an error with a message extracted from the response if the request fails.
 *
 * @param url The URL to make the request to.
 * @param options Extra options to pass to the Fetch API, such as method, headers, and body.
 * @returns The parsed JSON response from the server if the request is successful.
 */
async function request<T>(url: string, options?: RequestInit): Promise<T> {
  const response = await fetch(url, options);

  let data;

  try {
    data = await response.json();
  } catch {
    data = await response.text();
  }

  if (!response.ok) {
    throw new Error(
      (data as ErrorMessage | undefined)?.message || "Request failed",
    );
  }

  return data as T;
}

/**
 * Makes a GET request to the specified URL and returns the parsed JSON response.
 *
 * If the request fails, it throws an error with a message extracted from the response if
 * available, or a generic message if not.
 *
 * @param url The URL to make the request to.
 * @param options Extra options to pass to the Fetch API, such as headers and body.
 * @returns The parsed JSON response from the server if the request is successful.
 */
export function get<T>(url: string, options?: RequestInit): Promise<T> {
  return request(url, {
    method: "GET",
    ...options,
  });
}

/**
 * Makes a POST request to the specified URL and returns the parsed JSON response.
 *
 * If the request fails, it throws an error with a message extracted from the response if
 * available, or a generic message if not.
 *
 * @param url The URL to make the request to.
 * @param options Extra options to pass to the Fetch API, such as headers and body.
 * @returns The parsed JSON response from the server if the request is successful.
 */
export function post<T>(
  url: string,
  options?: Omit<RequestInit, "body" | "method"> & { body?: any },
): Promise<T> {
  return request(url, {
    method: "POST",
    headers: {
      ...(options?.body ? { "Content-Type": "application/json" } : {}),
      ...options?.headers,
    },
    ...options,
    body: options?.body ? JSON.stringify(options.body) : undefined,
  });
}

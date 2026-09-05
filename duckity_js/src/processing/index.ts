const RSA_MODULUS_SIZE = 512;

interface ChallengeMeta {
  // The challenge's ID.
  challenge_id: string;
  // The client's IP.
  ip: string;
  // The unix timestamp, in milliseconds, in which the challenge was issued.
  timestamp: number;
  // The ID of the protection profile this challenge was issued for.
  protection_profile_id: string;
  // The digits of the N Wesolowski VDF parameter, most significant digit first.
  n: number[];
  // The digits of the X Wesolowski VDF parameter, most significant digit first.
  x: number[];
  // The T Wesolowski VDF parameter.
  t: number;
}

export interface Challenge {
  // The challenge's ID.
  id: string;
  // The client's IP.
  ip: string;
  // N Wesolowski VDF parameter.
  n: bigint;
  // X Wesolowski VDF parameter.
  x: bigint;
  // T Wesolowski VDF parameter.
  t: number;
  // The timestamp at which this challenge was issued.
  timestamp: Date;
}

interface SolutionMeta {
  y: number[];
  pi: number[];
}

export interface Solution {
  y: bigint;
  pi: bigint;
}

/**
 * Converts an array of digits to a bigint.
 *
 * @param digits The digits of the bigint, MSF.
 * @param bits The amount of bits each digit consists of.
 * @returns The formed bigint.
 */
function getBigintFromDigits(digits: Array<number>, bits: bigint): bigint {
  let number = 0n;

  for (const digit of digits) {
    number = (number << bits) | BigInt(digit);
  }

  return number;
}

function getBigintWidth(number: bigint, alignment?: number): number {
  let bitWidth = 0;

  if (number !== 0n) {
    bitWidth = number.toString(2).length;
  }

  let byteWidth = Math.floor((bitWidth + 7) / 8);

  if (alignment !== undefined) {
    byteWidth = Math.ceil(byteWidth / alignment) * alignment;
  }

  return byteWidth;
}

/**
 * Returns a digit array from a big integer.
 *
 * @param number The number to get the digits from.
 * @param width The byte width of the result. If undefined, this will be the minimum width required
 * to represent the full number.
 * @param digitBits The amount of bits per digit.
 * @returns The digits, MSBF.
 */
function getDigitsFromBigint(
  number: bigint,
  width?: number,
  digitBits?: bigint,
): number[] {
  if (digitBits === undefined) {
    digitBits = 32n;
  }

  if (width === undefined) {
    width = getBigintWidth(number, 1);
  }

  let digits = width / Number(digitBits / 8n);
  let bytes: number[] = new Array(digits).fill(0);

  let mask = 0n;
  for (let i = digitBits; i > 0; i--) {
    mask <<= 1n;
    mask |= 1n;
  }

  for (let i = digits - 1; i >= 0; i--) {
    let byte = Number(number & mask);
    bytes[i] = byte;
    number = number >> digitBits;
  }

  return bytes;
}

function modPow(base: bigint, exponent: bigint, modulus: bigint): bigint {
  let result = 1n;

  while (exponent !== 0n) {
    // (something & 1n) is faster than (something % 2n)
    if ((exponent & 1n) === 1n) {
      result *= base;
      result %= modulus;
    }

    base **= 2n;
    base %= modulus;

    exponent /= 2n;
  }

  return result;
}

// Runs `tests` Miller-Rabin primality checks on `number`.
function isPrime(number: bigint, tests: number): boolean {
  if ([1n, 2n, 3n, 5n, 7n].includes(number)) {
    return true;
  }

  // (something & 1n) is faster than (something % 2n)
  if ((number & 1n) === 0n) {
    return false;
  }

  if (number % 5n === 0n) {
    return false;
  }

  let base = 2n;

  for (let i = 0n; i < BigInt(tests) && base + i < number; i++) {
    let prime = isPrimeForBase(number, base + i);

    if (!prime) {
      return false;
    }
  }

  return true;
}

function isPrimeForBase(number: bigint, base: bigint): boolean {
  let numberMinusOne = number - 1n;

  let odd = numberMinusOne;
  let baseTimes = 0;

  while (odd % 2n == 0n) {
    odd /= 2n;
    baseTimes += 1;
  }

  let oddPower = modPow(base, odd, number);

  for (let i = 0n; i < baseTimes; i++) {
    if (oddPower == 1n || oddPower == numberMinusOne) {
      return true;
    }

    oddPower = modPow(oddPower, 2n, number);
  }

  return false;
}

function getNextPrime(number: bigint): bigint {
  while (true) {
    number += 1n;

    if (isPrime(number, 40)) {
      return number;
    }
  }
}

/**
 * Extracts the challenge's data from the string.
 * 
 * @param challenge The raw challenge string.
 * @returns The decoded challenge metadata.
 */
export function decode(challenge: string): Challenge {
  // Counts all dots in the challenge string.
  if (![1, 2].includes((challenge.match(/\./g) || []).length)) {
    throw Error(
      "The challenge string contained too many or not enough sections.",
    );
  }

  let [base64, _signature] = challenge.split(".", 2);
  let json = atob(base64 as string);
  let data: ChallengeMeta = JSON.parse(json as string);

  let decoded: Challenge = {
    id: data.challenge_id,
    ip: data.ip,
    n: getBigintFromDigits(data.n, 32n),
    x: getBigintFromDigits(data.x, 32n),
    t: data.t,
    timestamp: new Date(data.timestamp),
  };

  return decoded;
}

/**
 * Solves a challenge and returns the solution to it.
 *
 * The solution must be encoded into the final submittable string with `encode()`.
 *
 * Always call this function from a worker. Not doing so will block the UI thread.
 *
 * @param challenge The decoded challenge string. See `decode()`.
 * @returns The solution to the provided challenge.
 */
export async function solve(challenge: Challenge): Promise<Solution> {
  let y = challenge.x;
  for (let i = 0; i < challenge.t; i++) {
    y = modPow(y, 2n, challenge.n);
  }

  let bytes = Array.from(new TextEncoder().encode("duckity"));
  bytes = bytes.concat(
    ...getDigitsFromBigint(challenge.n, RSA_MODULUS_SIZE, 8n),
  );
  bytes = bytes.concat(
    ...getDigitsFromBigint(challenge.x, RSA_MODULUS_SIZE, 8n),
  );
  bytes = bytes.concat(
    ...getDigitsFromBigint(BigInt(challenge.t), RSA_MODULUS_SIZE, 8n),
  );
  bytes = bytes.concat(...getDigitsFromBigint(y, RSA_MODULUS_SIZE, 8n));
  let hash = await crypto.subtle.digest("SHA-256", new Uint8Array(bytes));
  let hashBytes = Array.from(new Uint8Array(hash));

  let z = getBigintFromDigits(hashBytes, 8n);
  let l = getNextPrime(z);

  // We need 2^T = Q * L + R but 2^T is too big to calculate. Since 2^T is 2 << T in binary, we can
  // calculate PI on the go by keeping that in mind without actually storing the whole number in
  // memory.
  let r = 1n;
  let pi = 1n;

  for (let i = 0n; i < challenge.t; i++) {
    r = r * 2n;

    if (r >= l) {
      r = r - l;
      pi = (pi * pi * challenge.x) % challenge.n;
    } else {
      pi = (pi * pi) % challenge.n;
    }
  }

  return {
    y,
    pi,
  };
}

export function encode(original: string, solution: Solution): string {
  let solutionMeta: SolutionMeta = {
    y: getDigitsFromBigint(solution.y, 512, 32n),
    pi: getDigitsFromBigint(solution.pi, 512, 32n),
  };

  let solutionMetaJson = JSON.stringify(solutionMeta);
  let solutionMetaBase64 = btoa(solutionMetaJson)
    .replaceAll("=", "")
    .replaceAll("+", "-")
    .replaceAll("/", "_");

  return `${original}.${solutionMetaBase64}`;
}

export default {
  encode,
  solve,
  decode,
};

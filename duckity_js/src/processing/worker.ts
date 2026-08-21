import * as Comlink from "comlink";

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
  n: Array<number>;
  // The digits of the X Wesolowski VDF parameter, most significant digit first.
  x: Array<number>;
  // The T Wesolowski VDF parameter.
  t: number;
}

interface Challenge {
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

interface Solution {
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

  let byteWidth = Math.ceil((bitWidth + 7) / 8);

  if (alignment !== undefined) {
    byteWidth = Math.ceil(byteWidth / alignment) * alignment;
  }

  return byteWidth;
}

function getDigitsFromBigint(
  number: bigint,
  width?: number,
  digitBits?: bigint,
): number[] {
  let bytes: number[] = new Array(width).fill(0);

  if (digitBits === undefined) {
    digitBits = 32n;
  }

  if (width === undefined) {
    width = parseInt(
      (BigInt(number.toString(2).length) / digitBits).toString(),
    );
  }

  let mask = 0n;
  for (let i = digitBits - 1n; i > 0; i--) {
    mask <<= 1n;
    mask &= 1n;
  }

  for (let i = width - 1; i > 0; i--) {
    let byte = parseInt((number & mask).toString());
    number = number >> digitBits;
    bytes[i] = byte;
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
  let number_minus_one = number - 1n;

  let odd = number_minus_one;
  let base_times = 0;

  while (odd % 2n == 0n) {
    odd /= 2n;
    base_times += 1;
  }

  let odd_power = modPow(base, odd, number);

  for (let i = 0n; i < base_times; i++) {
    if (odd_power == 1n || odd_power == number_minus_one) {
      return true;
    }

    odd_power = modPow(odd_power, 2n, number);
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

function decode(challenge: string): Challenge {
  // Counts all dots in the challenge string
  if ((challenge.match(/\./g) || []).length != 1) {
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

async function solve(challenge: Challenge): Promise<Solution> {
  let y = challenge.x;

  for (let i = 0; i < challenge.t; i++) {
    y = y ** BigInt(2) % BigInt(challenge.t);
  }

  // Align to 4 bytes since each digit is a u32.
  let width = getBigintWidth(y, 4);

  let xBytes = getDigitsFromBigint(challenge.x, width, 8n);
  let yBytes = getDigitsFromBigint(y, width, 8n);

  let bytes = Array.from(new TextEncoder().encode("duckity"));
  bytes.concat(...xBytes);
  bytes.concat(...yBytes);
  let hash = await crypto.subtle.digest("SHA-256", new Uint8Array(bytes));
  let hashBytes = Array.from(new Uint8Array(hash));

  let z = getBigintFromDigits(hashBytes, 8n);
  let l = getNextPrime(z);

  let pi = 1n;
  let acc = challenge.x;
  let exp_mod_l = 1n;

  for (let i = 0; i < challenge.t; i++) {
    let doubled = exp_mod_l * 2n;

    if (doubled >= l) {
      pi = (pi * acc) % challenge.n;
      exp_mod_l = doubled - l;
    } else {
      exp_mod_l = doubled;
    }
  }

  return {
    y,
    pi,
  };
}

function encode(original: string, solution: Solution): string {
  let solutionMeta: SolutionMeta = {
    y: getDigitsFromBigint(solution.y, undefined, 32n),
    pi: getDigitsFromBigint(solution.pi, undefined, 32n),
  };

  let solutionMetaJson = JSON.stringify(solutionMeta);
  let solutionMetaBase64 = btoa(solutionMetaJson);

  return `${original}.${solutionMetaBase64}`;
}

/**
 * Solves a challenge in the background.
 */
const api = {
  async solve(challenge: string) {
    let decodedChallenge = decode(challenge);
    let solution = await solve(decodedChallenge);
    let solutionString = encode(challenge, solution);

    return solutionString;
  },
};

Comlink.expose(api);

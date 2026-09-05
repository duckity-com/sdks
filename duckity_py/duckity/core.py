import json

import base64

import hashlib

from datetime import datetime

from dataclasses import dataclass

import gmpy2


@dataclass
class Challenge:
    _raw: str

    id: str
    ip: str

    timestamp: datetime

    n: gmpy2.mpz
    x: gmpy2.mpz
    t: int

    def solve(self) -> str:
        solution = solve(self)
        solution = encode(self._raw, solution)

        return solution


@dataclass
class Solution:
    y: gmpy2.mpz
    pi: gmpy2.mpz


def get_mpz_from_digits(digits: list[int], digit_byte_width: int) -> gmpy2.mpz:
    """Converts a list of MSF digits to a `gmpy2.mpz`.

    Arguments:
        digits (list[int]): The unsigned 32-bit digits.
        digit_byte_width (int): The amount of bytes each digit carries.

    Returns:
        gmpy2.mpz: The resulting GMP number.
    """

    return gmpy2.mpz.from_bytes(b"".join(i.to_bytes(digit_byte_width) for i in digits))


def get_digits_from_mpz(number: gmpy2.mpz | int, array_width: int, digit_byte_width: int) -> list[int]:
    """Converts a MPZ integer to its MSF digits.

    Arguments:
        number (gmpy2.mpz | int): The number to convert to digits.
        array_width (int): The amount of digits to return, padded with 0s.
        digit_byte_width (int): The amount of bytes each digit will carry.

    Returns:
        list[int]: The digits.
    """

    digits = [0] * array_width

    mask = 0
    for i in range(digit_byte_width):
        mask <<= 8
        mask |= 0b1111_1111

    for i in range(array_width):
        digit = number & mask
        digits[array_width - 1 - i] = int(digit)
        number >>= digit_byte_width * 8

    return digits


def decode(challenge: str) -> Challenge:
    parts = challenge.split(".")

    # 2 parts for only a challenge, 3 parts for a solved challenge
    if len(parts) not in [2, 3]:
        raise ValueError("The challenge string passed did not have two section.")

    challenge_part = parts[0]

    # Re-add padding
    data = base64.urlsafe_b64decode(challenge_part + "=" * (-len(challenge_part) % 4))
    data = json.loads(data)

    n: list[int] = data["n"]
    x: list[int] = data["x"]
    t: int = data["t"]

    n = gmpy2.mpz.from_bytes(b"".join(b.to_bytes(4) for b in n))
    x = gmpy2.mpz.from_bytes(b"".join(b.to_bytes(4) for b in x))

    return Challenge(
        _raw=".".join(challenge.split(".")[:2]),  # Keep only the first two sections
        n=n,
        x=x,
        t=t,
        id=data["challenge_id"],
        ip=data["ip"],
        timestamp=datetime.fromtimestamp(data["timestamp"] / 1000),
    )


def solve(challenge: Challenge) -> Solution:
    y = challenge.x

    for _ in range(challenge.t):
        y = (y**2) % challenge.n

    hash = hashlib.sha256()
    hash.update(b"duckity")
    hash.update(get_digits_from_mpz(challenge.n, 512, 1))
    hash.update(get_digits_from_mpz(challenge.x, 512, 1))
    hash.update(get_digits_from_mpz(challenge.t, 512, 1))
    hash.update(get_digits_from_mpz(y, 512, 1))
    hash = hash.digest()

    l = gmpy2.mpz.from_bytes(hash, "big")
    l = gmpy2.next_prime(l)

    pi = gmpy2.mpz(1)
    acc = challenge.x
    exp_mod_l = gmpy2.mpz(1)

    for _ in range(challenge.t):
        doubled = exp_mod_l * 2

        if doubled >= l:
            pi = (pi * acc) % challenge.n
            exp_mod_l = doubled - l
        else:
            exp_mod_l = doubled

        acc = (acc**2) % challenge.n

    return Solution(y=y, pi=pi)


def encode(original: str, solution: Solution) -> str:
    y_digits = get_digits_from_mpz(solution.y, 512, 4)
    pi_digits = get_digits_from_mpz(solution.pi, 512, 4)

    solution = {"y": y_digits, "pi": pi_digits}
    solution = json.dumps(solution)
    solution: str = base64.urlsafe_b64encode(solution.encode("utf-8")).decode()
    solution: str = solution.replace("=", "")

    return f"{original}.{solution}"


def process(challenge: str) -> str:
    decoded = decode(challenge)
    solution = solve(decoded)
    encoded = encode(challenge, solution)

    return encoded

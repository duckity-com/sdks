import math

import json

import base64

import struct

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


def decode(challenge: str) -> Challenge:
    parts = challenge.split(".")

    if len(parts) != 2:
        raise ValueError("The challenge string passed did not have two section.")

    challenge_part, _ = parts

    # Readd padding
    data = base64.urlsafe_b64decode(challenge_part + "=" * (-len(challenge_part) % 4))
    data = json.loads(data)

    n: list[int] = data["n"]
    x: list[int] = data["x"]
    t: int = data["t"]

    n = gmpy2.mpz.from_bytes(b"".join(b.to_bytes(4) for b in n))
    x = gmpy2.mpz.from_bytes(b"".join(b.to_bytes(4) for b in x))

    return Challenge(
        _raw=challenge,
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

    width = (challenge.n.bit_length() + 7) // 8

    hash = hashlib.sha256()
    hash.update(b"duckity")
    hash.update(challenge.x.to_bytes(width, "big"))
    hash.update(y.to_bytes(width, "big"))
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


def verify(challenge: Challenge, solution: Solution) -> bool:
    n = challenge.n
    x = challenge.x
    t = challenge.t
    y = solution.y
    pi = solution.pi

    # y must be in the RSA group/range
    if not (0 <= x < n and 0 <= y < n and 0 <= pi < n):
        return False

    width = _aligned_bytes(n, 4)

    h = hashlib.sha256()
    h.update(b"duckity")
    h.update(x.to_bytes(width, "big"))
    h.update(y.to_bytes(width, "big"))

    l = gmpy2.mpz.from_bytes(h.digest(), "big")
    l = gmpy2.next_prime(l)

    r = gmpy2.powmod(2, t, l)

    return y == gmpy2.powmod(pi, l, n) * gmpy2.powmod(x, r, n) % n


def _aligned_bytes(value: int, alignment: int) -> bytes:
    return math.ceil((value.bit_length() + 7) // 8 / alignment) * alignment


def encode(original: str, solution: Solution) -> str:
    y_bytes = solution.y.to_bytes(_aligned_bytes(solution.y, 4))
    pi_bytes = solution.pi.to_bytes(_aligned_bytes(solution.pi, 4))

    y_digits: list[int] = list(struct.unpack(f">{math.ceil(len(y_bytes) / 4)}I", y_bytes))
    pi_digits: list[int] = list(struct.unpack(f">{math.ceil(len(pi_bytes) / 4)}I", pi_bytes))

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

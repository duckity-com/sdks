import time

import asyncio

import duckity
import duckity.core


# In a real world scenario the secret is not available to the client, validation is done
# server-side.
POLICY_ID = "nIjTGgQ-bCNPn-m153dMl"
APPLICATION_SECRET = "9L3QMV-iDrUhwNR2VAYDRGxeJLIupa-iKD5mUTsb9x9"


async def main():
    print("Welcome to the Duckity-py example!")
    print()
    print("This example will fetch a challenge from the Duckling API and solve it.")
    print(
        "It may take some seconds depending on the challenge, your internet connection, and your "
        "device."
    )
    print("Timings will be displayed at the end.")
    print()

    client = duckity.Client()

    print(f"Getting and solving challenge for policy with ID {POLICY_ID}...")

    challenge_start = time.perf_counter()
    solution = await client.solve(POLICY_ID)
    challenge_elapsed = time.perf_counter() - challenge_start

    challenge = duckity.core.decode(solution)

    print("Validating challenge...")

    # In async code with actual concurrency, unlike this example, move this call to a thread.
    is_valid_start = time.perf_counter()
    is_valid = await client.validate(solution, challenge.ip, APPLICATION_SECRET, POLICY_ID)
    is_valid_elapsed = time.perf_counter() - is_valid_start

    print(f"Challenge validated! Is valid? {is_valid}")

    print()
    print("Timings:")
    print(f"Issuance + solving:   {challenge_elapsed:.0004f}s")
    print(f"Validation: {is_valid_elapsed:.0004f}s")


asyncio.run(main())

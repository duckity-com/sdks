import time

import asyncio

import duckity


# In a real world scenario the secret is not available to the client, validation is done
# server-side.
PROTECTION_PROFILE_ID = "nIjTGgQ-bCNPn-m153dMl"
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

    print(f"Getting challenge for protection profile with ID {PROTECTION_PROFILE_ID}...")

    challenge_start = time.perf_counter()
    challenge = await client.issue_challenge(PROTECTION_PROFILE_ID)
    challenge_elapsed = time.perf_counter() - challenge_start

    print(f"Solving challenge with hardness set to {challenge.t}...")

    solution_start = time.perf_counter()
    solution = challenge.solve()
    solution_elapsed = time.perf_counter() - solution_start

    print("Validating challenge...")

    # In async code with actual concurrency, unlike this example, move this call to a thread.
    is_valid_start = time.perf_counter()
    is_valid = await client.validate(
        PROTECTION_PROFILE_ID, APPLICATION_SECRET, solution, challenge.ip
    )
    is_valid_elapsed = time.perf_counter() - is_valid_start

    print(f"Challenge validated! Is valid? {is_valid}")

    print()
    print("Timings:")
    print(f"Issuance:   {challenge_elapsed:.0004f}s")
    print(f"Solving:    {solution_elapsed:.0004f}s")
    print(f"Validation: {is_valid_elapsed:.0004f}s")


asyncio.run(main())

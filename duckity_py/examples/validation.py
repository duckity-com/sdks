import asyncio

import duckity
import duckity.core


# In a real world scenario the secret is not available to the client, validation is done
# server-side.
#
# Paste your own policy ID and application secret to validate using your own policy.
POLICY_ID = "nIjTGgQ-bCNPn-m153dMl"
APPLICATION_SECRET = "9L3QMV-iDrUhwNR2VAYDRGxeJLIupa-iKD5mUTsb9x9"


async def main():
    print("Welcome to the Duckity-py validation example!")
    print("Unlike the basic.py example, this example only validates a solution token.")
    print(
        "For simplicity, the IP is retrieved from the challenge's metadata. This is not recommended in production setups."
    )
    print()

    client = duckity.Client()

    solution = input("Paste the solution token: ").strip()
    challenge = duckity.core.decode(solution)

    print("Validating solution...")

    is_valid = await client.validate(solution, challenge.ip, APPLICATION_SECRET, POLICY_ID)

    print(f"Challenge validated! Is valid? {is_valid}")


asyncio.run(main())

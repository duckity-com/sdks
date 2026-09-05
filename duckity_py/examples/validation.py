import asyncio

import duckity
import duckity.core


# In a real world scenario the secret is not available to the client, validation is done
# server-side.
#
# Paste your own protection profile ID and application secret to validate using your own protection
# profile.
PROTECTION_PROFILE_ID = "idp-r4-dpQmaQYq1s4uAW"
APPLICATION_SECRET = "ifTQXNY4QApnnadnZazAqv4hj5rCbMW7lLN3OL1WyXD"


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

    # In async code with actual concurrency, unlike this example, move this call to a thread.
    is_valid = await client.validate(
        PROTECTION_PROFILE_ID, APPLICATION_SECRET, solution, challenge.ip
    )

    print(f"Challenge validated! Is valid? {is_valid}")


asyncio.run(main())

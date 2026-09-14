import asyncio

from concurrent.futures import ProcessPoolExecutor

import httpx

from duckity import core


_http = httpx.AsyncClient()
_executor = ProcessPoolExecutor()


class Client:
    """A client for Duckity's Duckling API."""

    _base_url = "https://api.duckity.com/d1"

    def __init__(self, base_url: str = None):
        if base_url is not None:
            self._base_url = base_url

    async def solve(self, protection_profile_id: str) -> str:
        """Gets a new challenge from the API and solves it.

        Args:
            protection_profile_id (str): The ID of the protection profile to get the challenge for.

        Returns:
            str: The solution token.
        """

        response = await _http.post(
            f"{self._base_url}/challenges/{protection_profile_id}/issue",
        )
        response.raise_for_status()

        data = response.json()
        original = data["challenge"]
        challenge = core.decode(original)

        loop = asyncio.get_running_loop()
        solution = await loop.run_in_executor(
            _executor,
            core.solve,
            challenge,
        )

        return core.encode(original, solution)

    async def validate(
        self, solution: str, ip: str, application_secret: str, protection_profile_id: str
    ) -> bool:
        """Validates a solution token.

        Args:
            solution (str): The encoded solution token.
            ip (str): The IP of the client that submitted the solution.
            application_secret (str): The application's secret.
            protection_profile_id (str): The protection profile ID for which this token was issued.

        Returns:
            bool: Whether the solution is valid.
        """

        response = await _http.post(
            f"{self._base_url}/challenges/{protection_profile_id}/validate",
            json={"solution": solution, "ip": ip},
            headers={"Authorization": f"Bearer {application_secret}"},
        )
        response.raise_for_status()

        data = response.json()

        return data["is_valid"]


_default_client = Client()

solve = _default_client.solve
validate = _default_client.validate


__ALL__ = ["Client", "solve", "validate"]

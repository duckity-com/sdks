import httpx

from duckity import core


_http = httpx.AsyncClient()


class Client:
    _base_url = "https://api.duckity.com/d1"

    def __init__(self, base_url: str = None):
        if base_url is not None:
            self._base_url = base_url

    async def issue_challenge(
        self, protection_profile_id: str, threat_correlation_keys: dict[str, str] = None
    ) -> "core.Challenge":
        """Gets a new challenge from the API.

        Args:
            protection_profile_id (str): The ID of the protection profile to get the challenge for.
            threat_correlation_keys (dict[str, str], optional): A map of threat correlation keys to
                values to use. Defaults to an empty map.

        Returns:
            Challenge: The newly issued challenge.
        """

        if threat_correlation_keys is None:
            threat_correlation_keys = dict()

        response = await _http.post(
            f"{self._base_url}/challenges/{protection_profile_id}/issue",
            json={"keys": threat_correlation_keys},
        )
        response.raise_for_status()

        data = response.json()

        return core.decode(data["challenge"])

    async def validate_challenge(
        self, protection_profile_id: str, application_secret: str, solution: str, ip: str
    ) -> bool:
        response = await _http.post(
            f"{self._base_url}/challenges/{protection_profile_id}/validate",
            json={"solution": solution, "ip": ip},
            headers={"Authorization": f"Bearer {application_secret}"},
        )
        response.raise_for_status()

        data = response.json()

        return data["is_valid"]

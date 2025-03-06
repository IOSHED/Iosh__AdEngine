import logging
from typing import Any, Dict, Tuple

import aiohttp


class LocationService:
    """A service class for retrieving location information from OpenStreetMap's Nominatim API.

    This class provides functionality to get city and country information based on latitude/longitude coordinates
    by making requests to the OpenStreetMap Nominatim reverse geocoding API.
    """

    base_url: str = "https://nominatim.openstreetmap.org/reverse"

    @classmethod
    async def get_city_country(
        cls, latitude: float, longitude: float
    ) -> Tuple[str, str]:
        """Get the city and country for given coordinates.

        Args:
            latitude (float): The latitude coordinate
            longitude (float): The longitude coordinate

        Returns:
            Tuple[str, str]: A tuple containing (city, country). Returns ("Неизвестно", "Неизвестно") if lookup fails.
        """
        try:
            async with aiohttp.ClientSession() as session:
                params = await cls._get_params_query(latitude, longitude)

                async with session.get(
                    cls.base_url,
                    params=params,
                    headers={"User-Agent": "TgAdEngine/1.0"},
                ) as response:
                    data = await response.json()

                    return await cls._parse_response(data)

        except Exception as e:
            logging.error(f"Geocoding error: {str(e)}")

            return "Неизвестно", "Неизвестно"

    @classmethod
    async def _get_params_query(
        cls, latitude: float, longitude: float
    ) -> Dict[str, Any]:
        """Generate query parameters for the Nominatim API request.

        Args:
            latitude (float): The latitude coordinate
            longitude (float): The longitude coordinate

        Returns:
            Dict[str, Any]: Dictionary containing the query parameters
        """
        return {
            "lat": latitude,
            "lon": longitude,
            "format": "jsonv2",
        }

    @staticmethod
    async def _parse_response(data: dict) -> Tuple[str, str]:
        """Parse the Nominatim API response to extract city and country information.

        Args:
            data (dict): The JSON response from the Nominatim API

        Returns:
            Tuple[str, str]: A tuple containing (city, country)
        """
        address = data.get("address", {})

        city_keys = ["city", "town", "village", "municipality"]

        city = next((address[key] for key in city_keys if key in address), "Неизвестно")

        country = address.get("country_code", "Неизвестно")

        return city, country

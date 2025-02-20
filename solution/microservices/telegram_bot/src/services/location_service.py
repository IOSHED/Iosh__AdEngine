import logging
from typing import Any, Dict, Tuple

import aiohttp


class LocationService:
    base_url: str = "https://nominatim.openstreetmap.org/reverse"

    @classmethod
    async def get_city_country(
        cls, latitude: float, longitude: float
    ) -> Tuple[str, str]:
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
        return {
            "lat": latitude,
            "lon": longitude,
            "format": "jsonv2",
        }

    @staticmethod
    async def _parse_response(data: dict) -> Tuple[str, str]:
        address = data.get("address", {})

        city_keys = ["city", "town", "village", "municipality"]

        city = next((address[key] for key in city_keys if key in address), "Неизвестно")

        country = address.get("country_code", "Неизвестно")

        return city, country

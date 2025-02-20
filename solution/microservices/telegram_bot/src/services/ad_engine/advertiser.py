import uuid
from typing import Optional

from src.services.ad_engine.schemas.advertiser import AdvertiserSchema
from src.services.http_serves_parser import HttpServesParser


class AdvertiserService(HttpServesParser):
    @classmethod
    async def create_advertiser(cls, advertiser: AdvertiserSchema) -> AdvertiserSchema:
        url = f"{cls._base_url}/api/advertiser/bulk"
        try:
            response = await cls._make_request(
                method="POST", url=url, json_body=[advertiser.model_dump()]
            )

            advertiser_data = response.json()
            return AdvertiserSchema(**advertiser_data[0])

        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to create advertiser: {str(e)}")

    @classmethod
    async def get_advertiser_by_id(
        cls, advertiser_id: uuid.UUID
    ) -> Optional[AdvertiserSchema]:
        url = f"{cls._base_url}/advertiser/{advertiser_id}"

        try:
            response = await cls._make_request(method="GET", url=url)

            if response is None:
                return None

            advertiser_data = response.json()
            return AdvertiserSchema(**advertiser_data)

        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to retrieve advertiser: {str(e)}")

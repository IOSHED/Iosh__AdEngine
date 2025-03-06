import uuid
from typing import Optional

from src.services.ad_engine.schemas.ads import AdsSchema
from src.services.http_serves_parser import HttpServesParser


class AdsService(HttpServesParser):
    @classmethod
    async def get_ads(cls, client_id: uuid.UUID) -> Optional[AdsSchema]:
        url = f"{cls._host_url}/ads"
        try:
            response = await cls._make_request(
                method="GET", url=url, params={"client_id": client_id}
            )
            if response is None:
                return None

            ads_data = response.json()
            return AdsSchema(**ads_data)

        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to get ads: {str(e)}")

    @classmethod
    async def click_ads(cls, ad_id: uuid.UUID, client_id: uuid.UUID) -> Optional[str]:
        url = f"{cls._host_url}/ads/{ad_id}/click"

        try:
            response = await cls._make_request(
                method="POST", url=url, json_body={"client_id": str(client_id)}
            )

            if response is None:
                return None

            return "ok"

        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to retrieve client: {str(e)}")

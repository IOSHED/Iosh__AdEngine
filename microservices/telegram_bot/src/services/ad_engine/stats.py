import uuid
from typing import List, Optional

from src.services.ad_engine.schemas.stats import StatDailyResponse, StatResponse
from src.services.http_serves_parser import HttpServesParser


class StatsService(HttpServesParser):
    @classmethod
    async def get_stat_campaign(cls, campaign_id: uuid.UUID) -> Optional[StatResponse]:
        url = f"{cls._host_url}/stats/campaigns/{campaign_id}"
        try:
            response = await cls._make_request(method="GET", url=url)
            if response is None:
                return None

            return StatResponse(**response.json())

        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to get stats: {str(e)}")

    @classmethod
    async def get_stat_campaign_daily(
        cls, campaign_id: uuid.UUID
    ) -> Optional[List[StatDailyResponse]]:
        url = f"{cls._host_url}/stats/campaigns/{campaign_id}/daily"
        try:
            response = await cls._make_request(method="GET", url=url)
            if response is None:
                return None

            return [StatDailyResponse(**data) for data in response.json()]

        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to get daily stats: {str(e)}")

    @classmethod
    async def get_stat_advertiser_daily(
        cls, advertiser_id: uuid.UUID
    ) -> Optional[List[StatDailyResponse]]:
        url = f"{cls._host_url}/stats/advertisers/{advertiser_id}/campaigns/daily"
        try:
            response = await cls._make_request(method="GET", url=url)
            if response is None:
                return None

            return [StatDailyResponse(**data) for data in response.json()]

        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to get daily stats: {str(e)}")

    @classmethod
    async def get_stat_advertiser(
        cls, advertiser_id: uuid.UUID
    ) -> Optional[StatResponse]:
        url = f"{cls._host_url}/stats/advertisers/{advertiser_id}/campaigns"
        try:
            response = await cls._make_request(method="GET", url=url)
            if response is None:
                return None

            return StatResponse(**response.json())

        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to get stats: {str(e)}")

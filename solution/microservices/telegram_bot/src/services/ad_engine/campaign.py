import uuid
from typing import List, Optional

from src.services.ad_engine.schemas.campaign import (
    CampaignSchema,
    CampaignsCreateRequest,
    CampaignsGenerateTextRequest,
    CampaignsUpdateRequest,
)
from src.services.http_serves_parser import HttpServesParser


class CampaignService(HttpServesParser):
    @classmethod
    async def create_campaign(
        cls, campaign: CampaignsCreateRequest, advertiser_id: uuid.UUID
    ) -> Optional[CampaignSchema]:
        url = f"{cls._base_url}/advertisers/{advertiser_id}/campaigns"
        try:
            response = await cls._make_request(
                method="POST", url=url, json_body=campaign
            )

            if response is None:
                return None

            return CampaignSchema(**response.json())

        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to create campaign: {str(e)}")

    @classmethod
    async def get_campaigns(
        cls, advertiser_id: uuid.UUID, size: int, page: int
    ) -> Optional[List[CampaignSchema]]:
        url = f"{cls._base_url}/advertisers/{advertiser_id}/campaigns"
        try:
            response = await cls._make_request(
                method="GET", url=url, params={"size": size, "page": page}
            )

            if response is None:
                return None

            campaign_data = response.json()

            return [CampaignSchema(**data) for data in campaign_data]

        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to get campaigns: {str(e)}")

    @classmethod
    async def get_campaign(
        cls, advertiser_id: uuid.UUID, campaign_id: uuid.UUID
    ) -> Optional[List[CampaignSchema]]:
        url = f"{cls._base_url}/advertisers/{advertiser_id}/campaigns/{campaign_id}"
        try:
            response = await cls._make_request(method="GET", url=url)

            if response is None:
                return None

            campaign_data = response.json()

            return CampaignSchema(**campaign_data)
        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to get campaign: {str(e)}")

    @classmethod
    async def update_campaign(
        cls,
        campaign: CampaignsUpdateRequest,
        advertiser_id: uuid.UUID,
        campaign_id: uuid.UUID,
    ) -> Optional[CampaignSchema]:
        url = f"{cls._base_url}/advertisers/{advertiser_id}/campaigns/{campaign_id}"
        try:
            response = await cls._make_request(
                method="PUT", url=url, json_body=campaign
            )

            if response is None:
                return None

            return CampaignSchema(**response.json())

        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to update campaign: {str(e)}")

    @classmethod
    async def delete_campaign(
        cls, advertiser_id: uuid.UUID, campaign_id: uuid.UUID
    ) -> Optional[str]:
        url = f"{cls._base_url}/advertisers/{advertiser_id}/campaigns/{campaign_id}"
        try:
            response = await cls._make_request(method="DELETE", url=url)

            if response is None:
                return None

            return "ok"

        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to delete campaign: {str(e)}")

    @classmethod
    async def text_generation(
        cls,
        text_generation: CampaignsGenerateTextRequest,
        advertiser_id: uuid.UUID,
        campaign_id: uuid.UUID,
    ) -> Optional[CampaignSchema]:
        url = f"{cls._base_url}/advertisers/{advertiser_id}/campaigns/{campaign_id}/generate_test"
        try:
            response = await cls._make_request(
                method="PATCH", url=url, json_body=text_generation
            )

            if response is None:
                return None

            return CampaignSchema(**response.json())

        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to update campaign: {str(e)}")

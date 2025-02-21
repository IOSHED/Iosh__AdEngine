from typing import Any, Dict
from uuid import UUID

from aiogram.types import CallbackQuery
from aiogram_dialog import DialogManager

from src.services.ad_engine.bundle_utils import generate_uuid_from_id
from src.services.ad_engine.campaign import CampaignService
from src.services.ad_engine.schemas.campaign import TargetingCampaignSchema
from src.services.ad_engine.stats import StatsService

PAGE = {}


class AdvertiserHandler:
    page: Dict[UUID, list] = {}

    @classmethod
    async def get_stats_advertiser(
        cls,
        dialog_manager: DialogManager,
        **kwargs,
    ) -> Dict[str, Any]:
        advertiser_id = generate_uuid_from_id(dialog_manager.event.from_user.id)

        stats = await StatsService.get_stat_advertiser(advertiser_id=advertiser_id)
        return {
            "impressions_count": stats.impressions_count,
            "clicks_count": stats.clicks_count,
            "conversion": stats.conversion,
            "spent_impressions": stats.spent_impressions,
            "spent_clicks": stats.spent_clicks,
            "spent_total": stats.spent_total,
        }

    @classmethod
    async def get_stats(
        cls,
        dialog_manager: DialogManager,
        **kwargs,
    ) -> Dict[str, Any]:
        advertiser_id = generate_uuid_from_id(dialog_manager.event.from_user.id)

        stats = await StatsService.get_stat_campaign(
            campaign_id=cls.page[advertiser_id][1]
        )
        return {
            "impressions_count": stats.impressions_count,
            "clicks_count": stats.clicks_count,
            "conversion": stats.conversion,
            "spent_impressions": stats.spent_impressions,
            "spent_clicks": stats.spent_clicks,
            "spent_total": stats.spent_total,
        }

    @classmethod
    async def delete_campaign(
        cls,
        callback: CallbackQuery,
        _widget: Any,
        manager: DialogManager,
    ) -> None:
        advertiser_id = generate_uuid_from_id(callback.from_user.id)
        await CampaignService.delete_campaign(
            advertiser_id=advertiser_id,
            campaign_id=cls.page[advertiser_id][1],
        )
        if cls.page[advertiser_id][0] <= 1:
            cls.page[advertiser_id] = None
            return
        cls.page[advertiser_id][0] -= 1

    @classmethod
    async def go_back_campaign(
        cls,
        callback: CallbackQuery,
        _widget: Any,
        manager: DialogManager,
    ) -> None:
        advertiser_id = generate_uuid_from_id(callback.from_user.id)
        if cls.page[advertiser_id][0] <= 1:
            return
        cls.page[advertiser_id][0] -= 1

    @classmethod
    async def go_next_campaign(
        cls,
        callback: CallbackQuery,
        _widget: Any,
        manager: DialogManager,
    ) -> None:
        advertiser_id = generate_uuid_from_id(callback.from_user.id)
        cls.page[advertiser_id][0] += 1

    @classmethod
    async def get_campaign_data(
        cls,
        dialog_manager: DialogManager,
        **kwargs,
    ) -> Dict[str, Any]:
        user = dialog_manager.event.from_user
        advertiser_id = generate_uuid_from_id(user.id)
        if cls.page.get(advertiser_id, None) is None:
            cls.page[advertiser_id] = [1, ""]

        current_page = cls.page[advertiser_id][0]

        campaigns_response = await CampaignService.get_campaigns(
            advertiser_id=advertiser_id,
            page=current_page,
            size=1,
        )

        campaign = campaigns_response[0][0]

        cls.page[advertiser_id][1] = campaign.campaign_id

        return {
            **campaign.model_dump(),
            **campaign.targeting.model_dump(),
            **(await cls.get_flags_target(campaign.targeting)),
            "num_campaign": current_page,
            "max_num_campaign": campaigns_response[1],
        }

    @classmethod
    async def get_flags_target(cls, target: TargetingCampaignSchema) -> Dict[str, Any]:
        return {
            "is_targeting_age_from": target.age_from is not None,
            "is_targeting_age_to": target.age_to is not None,
            "is_targeting_gender": target.gender is not None,
            "is_targeting_location": target.location is not None,
            "is_targeting_age": any([target.age_from, target.age_to]),
            "is_targeting": any(
                [target.age_from, target.age_to, target.gender, target.location]
            ),
        }

import logging
from typing import Any, Dict

from aiogram.types import CallbackQuery, Message
from aiogram_dialog import DialogManager

from src.services.ad_engine.bundle_utils import generate_uuid_from_id
from src.services.ad_engine.campaign import CampaignService
from src.services.ad_engine.schemas.campaign import (
    CampaignsCreateRequest,
    CampaignsGenerateTextRequest,
)


class CampaignInfoHandler:
    @classmethod
    async def generate_text(
        cls,
        message: Message,
        _source: Any,
        manager: DialogManager,
        *_arg,
        **_kwarg,
    ) -> None:
        selected_type = "".join(manager.find("getting_generate_text").get_checked())
        if selected_type in ["TEXTTITLE", "TITLETEXT"]:
            selected_type = "ALL"
        if selected_type == "":
            return

        advertiser_id = manager.dialog_data["advertiser_id"]

        campaign = await CampaignService.text_generation(
            CampaignsGenerateTextRequest(
                ad_text=None,
                ad_title=None,
                generate_type=selected_type,
            ),
            advertiser_id,
            manager.dialog_data["campaign_id"],
        )
        manager.dialog_data["ad_text"] = campaign.ad_text
        manager.dialog_data["ad_title"] = campaign.ad_title

    @classmethod
    async def get_generated_text(
        cls,
        dialog_manager: DialogManager,
        **_kwargs,
    ) -> Dict[str, Any]:
        logging.debug(dialog_manager.dialog_data["ad_text"])
        logging.debug(dialog_manager.dialog_data["ad_title"])
        return {
            "ad_text": dialog_manager.dialog_data["ad_text"],
            "ad_title": dialog_manager.dialog_data["ad_title"],
        }

    @classmethod
    async def create_campaign(
        cls,
        callback: CallbackQuery,
        _widget: Any,
        manager: DialogManager,
    ) -> None:
        try:
            data = await cls.get_view_campaign(manager)
            data["targeting"] = await cls.get_targeting_data(manager)

            campaign = CampaignsCreateRequest(**data)
            advertiser_id = generate_uuid_from_id(callback.from_user.id)
            manager.dialog_data["advertiser_id"] = str(advertiser_id)

            logging.debug(f"Create advertiser: {campaign}")

            campaign = await CampaignService.create_campaign(campaign, advertiser_id)
            manager.dialog_data["campaign_id"] = campaign.campaign_id

        except Exception as e:
            callback.message.answer(
                "❌ Произошла ошибка при создании рекламной кампании, попробуйте позже..."
            )
            logging.error(f"Error creating advertiser: {e}")
            await manager.back()

    @classmethod
    async def get_view_campaign(
        cls,
        dialog_manager: DialogManager,
        **_kwargs,
    ) -> Dict[str, Any]:
        return {
            "start_date": dialog_manager.dialog_data["start_date"],
            "end_date": dialog_manager.dialog_data["end_date"],
            "impressions_limit": dialog_manager.dialog_data["impressions_limit"],
            "clicks_limit": dialog_manager.dialog_data["clicks_limit"],
            "cost_per_impression": dialog_manager.dialog_data["cost_per_impression"],
            "cost_per_click": dialog_manager.dialog_data["cost_per_click"],
            "ad_title": dialog_manager.dialog_data["ad_title"],
            "ad_text": dialog_manager.dialog_data["ad_text"],
        }

    @classmethod
    async def get_view_form_campaign(
        cls,
        dialog_manager: DialogManager,
        **_kwargs,
    ) -> Dict[str, Any]:
        campaign = await cls.get_view_campaign(dialog_manager)
        campaign.update(await cls.get_targeting_data(dialog_manager))
        campaign.update(await cls.get_targeting_flags(dialog_manager))
        return campaign

    @classmethod
    async def get_targeting_data(cls, dialog_manager: DialogManager) -> Dict[str, Any]:
        return {
            "age_from": dialog_manager.dialog_data.get("age_from", None),
            "age_to": dialog_manager.dialog_data.get("age_to", None),
            "gender": dialog_manager.dialog_data.get("gender", None),
            "location": dialog_manager.dialog_data.get("location", None),
        }

    @classmethod
    async def get_targeting_flags(
        cls, dialog_manager: DialogManager
    ) -> Dict[str, bool]:
        return {
            "is_targeting": dialog_manager.dialog_data.get("is_targeting", None),
            "is_targeting_age": dialog_manager.dialog_data.get(
                "is_targeting_age", None
            ),
            "is_targeting_age_from": dialog_manager.dialog_data.get(
                "is_targeting_age_from", None
            ),
            "is_targeting_age_to": dialog_manager.dialog_data.get(
                "is_targeting_age_to", None
            ),
            "is_targeting_gender": dialog_manager.dialog_data.get(
                "is_targeting_gender", None
            ),
            "is_targeting_location": dialog_manager.dialog_data.get(
                "is_targeting_location", None
            ),
        }

    @classmethod
    async def save_start_date(
        cls,
        message: Message,
        _source: Any,
        manager: DialogManager,
        *_arg,
        **_kwarg,
    ) -> None:
        manager.dialog_data["start_date"] = int(message.text)
        await manager.next()

    @classmethod
    async def save_end_date(
        cls,
        message: Message,
        _source: Any,
        manager: DialogManager,
        *_arg,
        **_kwarg,
    ) -> None:
        end_date = int(message.text)

        if end_date < manager.dialog_data["start_date"]:
            await message.answer(
                "❌ Дата окончания кампании должна быть позже даты начала! Попробуйте ввести ещё раз 😁)"
            )
            return

        manager.dialog_data["end_date"] = end_date
        await manager.next()

    @classmethod
    async def save_impressions_limit(
        cls,
        message: Message,
        _source: Any,
        manager: DialogManager,
        *_arg,
        **_kwarg,
    ) -> None:
        manager.dialog_data["impressions_limit"] = int(message.text)
        await manager.next()

    @classmethod
    async def save_clicks_limit(
        cls,
        message: Message,
        _source: Any,
        manager: DialogManager,
        *_arg,
        **_kwarg,
    ) -> None:
        clicks_limit = int(message.text)

        if clicks_limit > manager.dialog_data["impressions_limit"]:
            await message.answer(
                "❌ Лимит на клики должен быть меньше, чем лимит на просмотры! Попробуйте ввести ещё раз 😁)"
            )
            return

        manager.dialog_data["clicks_limit"] = clicks_limit
        await manager.next()

    @classmethod
    async def save_cost_per_impression(
        cls,
        message: Message,
        _source: Any,
        manager: DialogManager,
        *_arg,
        **_kwarg,
    ) -> None:
        manager.dialog_data["cost_per_impression"] = float(message.text)
        await manager.next()

    @classmethod
    async def save_cost_per_click(
        cls,
        message: Message,
        _source: Any,
        manager: DialogManager,
        *_arg,
        **_kwarg,
    ) -> None:
        manager.dialog_data["cost_per_click"] = float(message.text)
        await manager.next()

    @classmethod
    async def save_targeting_age_from(
        cls,
        message: Message,
        _source: Any,
        manager: DialogManager,
        *_arg,
        **_kwarg,
    ) -> None:
        if int(message.text) > 150:
            await message.answer(
                "❌ Возраст должен быть меньше 150! Попробуйте ввести ещё раз 😁)"
            )
            return

        manager.dialog_data["is_targeting"] = True
        manager.dialog_data["is_targeting_age"] = True
        manager.dialog_data["is_targeting_age_from"] = True
        manager.dialog_data["age_from"] = int(message.text)
        await manager.next()

    @classmethod
    async def save_targeting_age_to(
        cls,
        message: Message,
        _source: Any,
        manager: DialogManager,
        *_arg,
        **_kwarg,
    ) -> None:
        age_to = int(message.text)
        if age_to > 160:
            await message.answer(
                "❌ Возраст должен быть меньше 160! Попробуйте ввести ещё раз 😁)"
            )
            return

        if age_to < manager.dialog_data["age_from"]:
            await message.answer(
                "❌ Возраст 'до' должен быть больше, чем возраст 'от'! Попробуйте ввести ещё раз 😁)"
            )
            return

        manager.dialog_data["is_targeting"] = True
        manager.dialog_data["is_targeting_age"] = True
        manager.dialog_data["is_targeting_age_to"] = True
        manager.dialog_data["age_to"] = age_to
        await manager.next()

    @classmethod
    async def save_targeting_gender(
        cls,
        _message: Message,
        _widget: Any,
        manager: DialogManager,
    ) -> None:
        selected_gender = manager.find("getting_user_gender").get_checked()
        logging.debug(f"Parse interests: {selected_gender}")
        if selected_gender is not None:
            manager.dialog_data["is_targeting"] = True
            manager.dialog_data["is_targeting_gender"] = True
            manager.dialog_data["gender"] = selected_gender

    @classmethod
    async def save_targeting_location(
        cls,
        message: Message,
        _source: Any,
        manager: DialogManager,
        *_arg,
        **_kwarg,
    ) -> None:
        location = message.text
        manager.dialog_data["is_targeting"] = True
        manager.dialog_data["is_targeting_location"] = True
        manager.dialog_data["location"] = location
        await manager.next()

    @classmethod
    async def save_ad_title(
        cls,
        message: Message,
        _source: Any,
        manager: DialogManager,
        *_arg,
        **_kwarg,
    ) -> None:
        title = message.text
        manager.dialog_data["ad_title"] = title
        await manager.next()

    @classmethod
    async def save_ad_text(
        cls,
        message: Message,
        _source: Any,
        manager: DialogManager,
        *_arg,
        **_kwarg,
    ) -> None:
        text = message.text
        manager.dialog_data["ad_text"] = text
        await manager.next()

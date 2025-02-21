import logging
from typing import Any, Dict
from uuid import UUID

from aiogram.types import CallbackQuery
from aiogram_dialog import DialogManager

from src.services.ad_engine.ads import AdsService
from src.services.ad_engine.bundle_utils import generate_uuid_from_id


class AdsHandler:
    click_data: Dict[UUID, UUID] = {}

    @classmethod
    async def click(
        cls,
        callback: CallbackQuery,
        _widget: Any,
        manager: DialogManager,
        **_kwargs,
    ) -> None:
        client_id = generate_uuid_from_id(manager.event.from_user.id)
        ad_id = cls.click_data[client_id]
        await AdsService.click_ads(ad_id, client_id)
        logging.info(f"clicked ad {ad_id} for user {client_id}")
        await callback.answer("👍🎈Спасибо за отклик!")

    @classmethod
    async def get_ads(
        cls,
        dialog_manager: DialogManager,
        **_kwargs,
    ) -> Dict[str, Any]:
        client_id = generate_uuid_from_id(dialog_manager.event.from_user.id)

        ads = await AdsService.get_ads(str(client_id))

        if ads is None:
            logging.info(f"404 for user {client_id}")
            return {
                "ad_text": "❌ Нет подходящей для вас рекламы.",
                "ad_title": "Попробуйте позже..",
                "ad_id": "Или разместите рекламу сами!😊",
            }

        cls.click_data[client_id] = ads.ad_id

        return {
            "ad_text": ads.ad_text.replace("*", ""),
            "ad_title": ads.ad_title.replace("*", ""),
            "advertiser_id": ads.advertiser_id,
            "ad_id": ads.ad_id,
        }

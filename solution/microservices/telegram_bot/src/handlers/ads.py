from typing import Any, Dict

from aiogram_dialog import DialogManager

from src.services.ad_engine.ads import AdsService
from src.services.ad_engine.bundle_utils import generate_uuid_from_id


class AdsHandler:
    @classmethod
    async def get_ads(
        cls,
        dialog_manager: DialogManager,
        **_kwargs,
    ) -> Dict[str, Any]:
        client_id = generate_uuid_from_id(dialog_manager.event.from_user.id)

        ads = await AdsService.get_ads(str(client_id))

        if ads is None:
            return {
                "ad_text": "❌ Нет подходящей для вас рекламы.",
                "ad_title": "Попробуйте позже..",
                "advertiser_id": "Или разместите рекламу сами!😊",
            }

        return {
            "ad_text": ads.ad_text,
            "ad_title": ads.ad_title,
            "advertiser_id": ads.advertiser_id,
        }

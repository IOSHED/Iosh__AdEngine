import logging
from typing import Any, Dict

from aiogram.types import CallbackQuery, Message
from aiogram_dialog import DialogManager

from src.services.ad_engine.advertiser import AdvertiserService
from src.services.ad_engine.bundle_utils import generate_uuid_from_id
from src.services.ad_engine.schemas.advertiser import AdvertiserSchema


class AdvertiserInfoHandler:
    @classmethod
    async def create_advertiser(
        cls,
        callback: CallbackQuery,
        _widget: Any,
        manager: DialogManager,
    ) -> None:
        try:
            advertiser = AdvertiserSchema(
                advertiser_id=str(generate_uuid_from_id(callback.from_user.id)),
                name=manager.dialog_data["name"],
            )

            logging.debug(f"Create advertiser: {advertiser}")

            await AdvertiserService.create_advertiser(advertiser)

        except Exception as e:
            logging.error(f"Error creating advertiser: {e}")
            await manager.back()
            callback.answer(
                "❌ Произошла ошибка при создании пользователя, попробуйте позже..."
            )

    @classmethod
    async def get_view_form_advertiser(
        cls,
        dialog_manager: DialogManager,
        **_kwargs,
    ) -> Dict[str, Any]:
        return {
            "name": dialog_manager.dialog_data["name"],
        }

    @classmethod
    async def save_name(
        cls,
        message: Message,
        _source: Any,
        manager: DialogManager,
        *_arg,
        **_kwarg,
    ) -> None:
        name = message.text.strip()
        manager.dialog_data["name"] = name
        await manager.next()

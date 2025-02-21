import logging
from typing import Any

from aiogram.types import CallbackQuery
from aiogram_dialog import DialogManager

from src.dialogs.advertiser_info import AdvertiserInfoDialog
from src.dialogs.advertisers import AdvertiserDialog
from src.services.ad_engine.advertiser import AdvertiserService
from src.services.ad_engine.bundle_utils import generate_uuid_from_id


class AdvertiserCheckHandler:
    @classmethod
    async def check_exists(
        cls,
        callback: CallbackQuery,
        _widget: Any,
        manager: DialogManager,
    ) -> None:
        user_id = callback.from_user.id
        logging.debug(f"Get advertiser: {user_id}")
        if (
            await AdvertiserService.get_advertiser_by_id(generate_uuid_from_id(user_id))
            is None
        ):
            await manager.start(AdvertiserInfoDialog.home)
            return
        await manager.start(AdvertiserDialog.home)

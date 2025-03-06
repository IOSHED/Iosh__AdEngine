import logging
from typing import Any

from aiogram.types import CallbackQuery
from aiogram_dialog import DialogManager

from src.services.ad_engine.time import TimeService


class TimeHandler:
    @classmethod
    async def set_time_advance(
        cls,
        callback: CallbackQuery,
        _widget: Any,
        manager: DialogManager,
    ) -> None:
        time = manager.find("counter_getting_time_advance").get_value()
        logging.debug(f"Parse time advance: {time}")
        try:
            await TimeService.set(int(time))
        except Exception as e:
            logging.error(f"Error creating user: {e}")
            await manager.back()
            callback.answer(
                "❌ Произошла ошибка при промотке времени, попробуйте позже..."
            )

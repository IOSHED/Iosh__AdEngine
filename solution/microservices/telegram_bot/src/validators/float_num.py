from typing import Any

from aiogram.types import Message
from aiogram_dialog import (
    DialogManager,
)


class FloatNumValidator:
    @classmethod
    def validate(cls, text: str) -> str:
        float(text)
        return text

    @classmethod
    async def error(
        cls, message: Message, _dialog: Any, _manager: DialogManager, _error: ValueError
    ) -> None:
        await message.answer(
            "Вы ввели не <b>не число</b>! Попробуйте ввести ещё раз 😁)"
        )

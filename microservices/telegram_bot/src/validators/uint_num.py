from typing import Any

from aiogram.types import Message
from aiogram_dialog import (
    DialogManager,
)


class UIntNumValidator:
    @classmethod
    def validate(cls, text: str) -> str:
        if int(text) < 1:
            raise ValueError
        return text

    @classmethod
    async def error(
        cls, message: Message, _dialog: Any, _manager: DialogManager, _error: ValueError
    ) -> None:
        await message.answer(
            "Вы ввели не <b>целочисленное положительное значение</b>! Попробуйте ввести ещё раз 😁)"
        )

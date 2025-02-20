from typing import Any, List

from aiogram.types import Message
from aiogram_dialog import (
    DialogManager,
)


class ListWordsValidator:
    @classmethod
    def validate(cls, text: str) -> List[str]:
        words = text.split(",")
        for word in words:
            if len(word) > 32:
                raise ValueError("Word too long")

        return words

    @classmethod
    async def error(
        cls, message: Message, _dialog: Any, _manager: DialogManager, _error: ValueError
    ) -> None:
        await message.answer(
            "Вы ввели слишком большое <b>слово</b>! Попробуйте ввести слово не длиннее 32 символов😁)"
        )

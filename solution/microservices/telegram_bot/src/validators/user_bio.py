from typing import Any

from aiogram.types import Message
from aiogram_dialog import (
    DialogManager,
)


class BioValidator:
    """
    Validator class for handling user bio text validation.

    This class provides methods to validate bio text length and handle validation errors.
    """

    @classmethod
    def validate(cls, text: str) -> str:
        """
        Validates the bio text length.

        Args:
            text (str): The bio text to validate

        Returns:
            str: The validated bio text if validation passes

        Raises:
            ValueError: If bio text length exceeds 255 characters
        """
        if len(text) > 255:
            raise ValueError("Bio too long")
        return text

    @classmethod
    async def error(
        cls, message: Message, _dialog: Any, _manager: DialogManager, _error: ValueError
    ) -> None:
        """
        Handles bio validation error by sending error message to user.

        Args:
            message (Message): The message object to reply to
            _dialog (Any): Unused dialog parameter
            _manager (DialogManager): Unused dialog manager parameter
            _error (ValueError): The validation error that occurred

        Returns:
            None
        """
        await message.answer(
            "Вы ввели слишком большое <b>bio</b>! Попробуйте ввести меньше 255 символов😁)"
        )

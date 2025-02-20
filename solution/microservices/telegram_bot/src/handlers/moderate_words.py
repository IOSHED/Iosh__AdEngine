import csv
import io
import logging
from typing import Any

from aiogram.types import BufferedInputFile, CallbackQuery, Message
from aiogram_dialog import DialogManager

from src.services.ad_engine.moderate import ModerateService
from src.services.ad_engine.schemas.moderate import ModerateSchema


class ModerateWordsHandler:
    @classmethod
    async def change_moderate_words(
        cls,
        callback: CallbackQuery,
        widget: Any,
        manager: DialogManager,
        *_arg,
        **_kwarg,
    ) -> None:
        try:
            schema = ModerateSchema(
                is_activate=True if widget.get_checked() == "Выключить" else False
            )
            await ModerateService.set_moderate_settings(schema)

        except Exception as e:
            logging.error(f"Error get words: {e}")
            await manager.back()
            callback.answer(
                "❌ Произошла ошибка при получении листа, попробуйте позже..."
            )

    @classmethod
    async def get_words(
        cls,
        callback: CallbackQuery,
        _widget: Any,
        manager: DialogManager,
    ) -> None:
        try:
            words = await ModerateService.get_black_list_words()
            output = io.StringIO()
            writer = csv.writer(output)

            for word in words:
                writer.writerow([word])

            csv_data = output.getvalue().encode()

            await callback.message.answer_document(
                BufferedInputFile(csv_data, "black_words.csv"),
            )

        except Exception as e:
            logging.error(f"Error get words: {e}")
            await manager.back()
            callback.answer(
                "❌ Произошла ошибка при получении листа, попробуйте позже..."
            )

    @classmethod
    async def add_words(
        cls,
        message: Message,
        _source: Any,
        manager: DialogManager,
        *_arg,
        **_kwarg,
    ) -> None:
        try:
            words = message.text.replace(" ", "").split(",")
            await ModerateService.add_black_list_words(words)

        except Exception as e:
            logging.error(f"Error add words: {e}")
            await manager.back()
            message.answer(
                "❌ Произошла ошибка при добавлении слов в лист, попробуйте позже..."
            )

    @classmethod
    async def delete_words(
        cls,
        message: Message,
        _source: Any,
        manager: DialogManager,
        *_arg,
        **_kwarg,
    ) -> None:
        try:
            words = message.text.replace(" ", "").split(",")
            await ModerateService.delete_black_list_words(words)

        except Exception as e:
            logging.error(f"Error delete words: {e}")
            message.answer(
                "❌ Произошла ошибка при удалении слов в лист, попробуйте позже..."
            )
            await manager.back()

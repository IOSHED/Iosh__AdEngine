from aiogram.filters.state import State, StatesGroup
from aiogram_dialog import (
    Dialog,
    Window,
)
from aiogram_dialog.widgets.input import TextInput
from aiogram_dialog.widgets.kbd import Button, Cancel, SwitchTo, Toggle
from aiogram_dialog.widgets.text import Const, Format

from src.handlers.moderate_words import ModerateWordsHandler
from src.keyboards.moderate_words import BTN_MANAGE_WINDOW
from src.validators.list_words import ListWordsValidator


class ModerateWordsDialog(StatesGroup):
    main = State()
    view_words = State()
    delete_words = State()
    add_words = State()


moderate_words_dialog = Dialog(
    Window(
        Const("🤬 Модерация текста"),
        Toggle(
            Format("{item[0]} {item[1]}"),
            id="radio",
            items=[("✓", "Выключить"), ("", "Включить")],
            item_id_getter=lambda item: item[1],
            on_state_changed=ModerateWordsHandler.change_moderate_words,
        ),
        SwitchTo(
            Const("📝 Просмотреть нецензурные слова"),
            id="go_to_view_words",
            state=ModerateWordsDialog.view_words,
        ),
        SwitchTo(
            Const("➕ Добавить нецензурные слова"),
            id="go_to_add_words",
            state=ModerateWordsDialog.add_words,
        ),
        SwitchTo(
            Const("➖ Удалить нецензурные слова"),
            id="go_to_delete_words",
            state=ModerateWordsDialog.delete_words,
        ),
        Cancel(Const("🔙 Назад")),
        state=ModerateWordsDialog.main,
    ),
    Window(
        Const("📝 Просмотрите нецензурные слова в файле"),
        Button(
            Const("Получить файл со словами"),
            id="getter_file_with_black_words",
            on_click=ModerateWordsHandler.get_words,
        ),
        BTN_MANAGE_WINDOW(ModerateWordsDialog.main),
        state=ModerateWordsDialog.view_words,
    ),
    Window(
        Const("➕ <b>Добавить нецензурные слова</b>"),
        Const("Вводите слова, которые хотите добавить, через запятую."),
        BTN_MANAGE_WINDOW(ModerateWordsDialog.main),
        TextInput(
            id="new_black_words",
            on_error=ListWordsValidator.error,
            type_factory=ListWordsValidator.validate,
            on_success=ModerateWordsHandler.add_words,
        ),
        state=ModerateWordsDialog.add_words,
    ),
    Window(
        Const("➖ Удалить нецензурные слова"),
        Const("Вводите слова, которые хотите удалить, через запятую."),
        BTN_MANAGE_WINDOW(ModerateWordsDialog.main),
        TextInput(
            id="delete_black_words",
            on_error=ListWordsValidator.error,
            type_factory=ListWordsValidator.validate,
            on_success=ModerateWordsHandler.delete_words,
        ),
        state=ModerateWordsDialog.delete_words,
    ),
)

from aiogram.filters.state import State, StatesGroup
from aiogram_dialog import (
    Dialog,
    Window,
)
from aiogram_dialog.widgets.input import TextInput
from aiogram_dialog.widgets.kbd import Back, Button, Cancel, SwitchTo, Toggle
from aiogram_dialog.widgets.text import Const, Format

from src.buttons.moderate_words import BTN_COUNTER_TIME_ADVANCE, BTN_MANAGE_WINDOW
from src.handlers.moderate_words import ModerateWordsHandler
from src.handlers.time import TimeHandler
from src.messages.moderate import MSG_ADMIN_PANEL
from src.validators.list_words import ListWordsValidator


class ModerateDialog(StatesGroup):
    home = State()
    main_moderate_text = State()
    view_words = State()
    delete_words = State()
    add_words = State()
    set_time_advance = State()


moderate_words_dialog = Dialog(
    Window(
        MSG_ADMIN_PANEL,
        SwitchTo(
            Const("🕐 Промотать время"),
            id="go_to_time_advance",
            state=ModerateDialog.set_time_advance,
        ),
        SwitchTo(
            Const("🤬 Модерация текста"),
            id="go_to_moderate",
            state=ModerateDialog.main_moderate_text,
        ),
        Cancel(Const("🏠 На главную")),
        state=ModerateDialog.home,
    ),
    Window(
        Const("🕐 Промотать время до"),
        BTN_COUNTER_TIME_ADVANCE,
        Back(
            Const("✅ Подтвердить"),
            id="confirm_time_advance",
            on_click=TimeHandler.set_time_advance,
        ),
        BTN_MANAGE_WINDOW(ModerateDialog.home),
        state=ModerateDialog.set_time_advance,
    ),
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
            state=ModerateDialog.view_words,
        ),
        SwitchTo(
            Const("➕ Добавить нецензурные слова"),
            id="go_to_add_words",
            state=ModerateDialog.add_words,
        ),
        SwitchTo(
            Const("➖ Удалить нецензурные слова"),
            id="go_to_delete_words",
            state=ModerateDialog.delete_words,
        ),
        BTN_MANAGE_WINDOW(ModerateDialog.home),
        state=ModerateDialog.main_moderate_text,
    ),
    Window(
        Const("📝 Просмотрите нецензурные слова в файле"),
        Button(
            Const("Получить файл со словами"),
            id="getter_file_with_black_words",
            on_click=ModerateWordsHandler.get_words,
        ),
        BTN_MANAGE_WINDOW(ModerateDialog.main_moderate_text),
        state=ModerateDialog.view_words,
    ),
    Window(
        Const("➕ <b>Добавить нецензурные слова</b>"),
        Const("Вводите слова, которые хотите добавить, через запятую."),
        BTN_MANAGE_WINDOW(ModerateDialog.main_moderate_text),
        TextInput(
            id="new_black_words",
            on_error=ListWordsValidator.error,
            type_factory=ListWordsValidator.validate,
            on_success=ModerateWordsHandler.add_words,
        ),
        state=ModerateDialog.add_words,
    ),
    Window(
        Const("➖ Удалить нецензурные слова"),
        Const("Вводите слова, которые хотите удалить, через запятую."),
        BTN_MANAGE_WINDOW(ModerateDialog.main_moderate_text),
        TextInput(
            id="delete_black_words",
            on_error=ListWordsValidator.error,
            type_factory=ListWordsValidator.validate,
            on_success=ModerateWordsHandler.delete_words,
        ),
        state=ModerateDialog.delete_words,
    ),
)

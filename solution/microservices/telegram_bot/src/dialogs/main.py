from aiogram.filters.state import State, StatesGroup
from aiogram_dialog import (
    Dialog,
    Window,
)
from aiogram_dialog.widgets.text import Const


class MainDialog(StatesGroup):
    main = State()


main_dialog = Dialog(
    Window(
        Const("Main"),
        state=MainDialog.main,
    ),
)

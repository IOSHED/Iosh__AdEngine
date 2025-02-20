from aiogram.filters.state import State
from aiogram_dialog.widgets.kbd import Cancel, Row, SwitchTo
from aiogram_dialog.widgets.text import Const


def BTN_MANAGE_WINDOW(state: State) -> Row:
    return Row(
        SwitchTo(
            Const("🔙 Назад"),
            id="go_to_main_moderate",
            state=state,
        ),
        Cancel(Const("⛔ На главную")),
    )

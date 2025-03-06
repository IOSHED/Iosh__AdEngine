from aiogram.filters.state import State
from aiogram_dialog.widgets.kbd import Button, Cancel, Counter, Row, SwitchTo
from aiogram_dialog.widgets.text import Const, Format


def BTN_MANAGE_WINDOW(state: State) -> Row:
    return Row(
        SwitchTo(
            Const("🔙 Назад"),
            id="go_to_main_moderate",
            state=state,
        ),
        Cancel(Const("🏠 На главную")),
    )


BTN_COUNTER_TIME_ADVANCE = Row(
    Counter(
        id="counter_getting_time_advance",
        plus=Const(">"),
        minus=Const("<"),
        text=Format("{value:g}"),
        min_value=0,
    ),
    Button(
        Const(">>"),
        id="counter_first",
        on_click=lambda _callback, _button, manager: manager.find(
            "counter_getting_time_advance"
        ).set_value(manager.find("counter_getting_time_advance").get_value() + 5),
    ),
)

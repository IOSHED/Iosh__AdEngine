from aiogram_dialog.widgets.kbd import Button, Counter, Row
from aiogram_dialog.widgets.text import Const, Format


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

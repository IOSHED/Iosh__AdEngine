import operator

from aiogram_dialog.widgets.kbd import Button, Counter, Radio, RequestLocation, Row
from aiogram_dialog.widgets.text import Const, Format

BTN_GETTING_LOCATION = RequestLocation(Const("📍 Отправить геопозицию"))

BTN_GET_USER_GENDER = Row(
    Radio(
        Format("✓ {item[0]}"),
        Format("{item[0]}"),
        id="getting_user_gender",
        item_id_getter=operator.itemgetter(1),
        items=[("🚹 Мужчина", "MALE"), ("🚺 Женщина", "FEMALE")],
    )
)

BTN_GET_AGE = Row(
    Button(
        Const("<<"),
        id="counter_last",
        on_click=lambda _callback, _button, manager: manager.find(
            "counter_getting_age"
        ).set_value(max(manager.find("counter_getting_age").get_value() - 5, 1)),
    ),
    Counter(
        id="counter_getting_age",
        plus=Const(">"),
        minus=Const("<"),
        default=20,
        min_value=1,
        max_value=100,
        cycle=True,
    ),
    Button(
        Const(">>"),
        id="counter_first",
        on_click=lambda _callback, _button, manager: manager.find(
            "counter_getting_age"
        ).set_value(min(manager.find("counter_getting_age").get_value() + 5, 100)),
    ),
)

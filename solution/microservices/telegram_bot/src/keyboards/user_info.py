import operator

from aiogram_dialog.widgets.kbd import Column, Multiselect, RequestLocation
from aiogram_dialog.widgets.text import Const, Format

BTN_GETTING_LOCATION = RequestLocation(Const("📍 Отправить геопозицию"))

KEYBOARD_GETTING_INTERESTS = Column(
    Multiselect(
        Format("✓ {item[0]}"),
        Format("{item[0]}"),
        id="getting_list_interests",
        item_id_getter=operator.itemgetter(1),
        items="list_interests",
    )
)

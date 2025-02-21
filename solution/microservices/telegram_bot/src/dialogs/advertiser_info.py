from enum import Enum

from aiogram.filters.state import State, StatesGroup
from aiogram_dialog import (
    Dialog,
    Window,
)
from aiogram_dialog.widgets.input import TextInput
from aiogram_dialog.widgets.kbd import Cancel, Next, SwitchTo
from aiogram_dialog.widgets.text import Const

from src.handlers.advertiser_info import AdvertiserInfoHandler
from src.handlers.calc_steps import CalkStepsHandler
from src.messages.advertiser_info import MSG_FIRST, MSG_GET_NAME, MSG_VIEW_FORM


class StepsAddInfo(Enum):
    NAME = "name"


class AdvertiserInfoDialog(StatesGroup):
    home = State()
    get_name = State()
    view_form = State()


advertiser_info_dialog = Dialog(
    Window(
        MSG_FIRST,
        Next(Const("✅ Продолжить"), id="return_to_name"),
        state=AdvertiserInfoDialog.home,
    ),
    Window(
        MSG_GET_NAME,
        TextInput(
            id="name",
            on_success=AdvertiserInfoHandler.save_name,
        ),
        state=AdvertiserInfoDialog.get_name,
        getter=CalkStepsHandler(StepsAddInfo).get_steps,
    ),
    Window(
        MSG_VIEW_FORM,
        Cancel(
            Const("✅ Продолжить"),
            on_click=AdvertiserInfoHandler.create_advertiser,
        ),
        SwitchTo(
            Const("🔙 Заполнить заново"),
            id="return_to_name",
            state=AdvertiserInfoDialog.get_name,
        ),
        getter=AdvertiserInfoHandler.get_view_form_advertiser,
        state=AdvertiserInfoDialog.view_form,
    ),
)

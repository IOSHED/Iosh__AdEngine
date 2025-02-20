from enum import Enum

from aiogram.filters.state import State, StatesGroup
from aiogram.types import ContentType
from aiogram_dialog import Dialog, Window
from aiogram_dialog.widgets.input import MessageInput
from aiogram_dialog.widgets.kbd import Next, Start, SwitchTo
from aiogram_dialog.widgets.markup.reply_keyboard import ReplyKeyboardFactory
from aiogram_dialog.widgets.text import Const

from src.dialogs.main import MainDialog
from src.handlers.calc_steps import CalkStepsHandler
from src.handlers.user_info import UserInfoHandler
from src.keyboards.user_info import (
    BTN_GET_AGE,
    BTN_GET_USER_GENDER,
    BTN_GETTING_LOCATION,
)
from src.messages.base import PLACEHOLDER_FOR_MURK_UP
from src.messages.user_info import (
    MSG_FIRST,
    MSG_GET_AGE,
    MSG_GET_GENDER,
    MSG_GET_LOCATION,
    MSG_VIEW_FORM,
)


class StepsAddInfo(Enum):
    AGE = "age"
    GENDER = "gender"
    LOCATION = "location"


class UserInfo(StatesGroup):
    preview = State()
    get_age = State()
    get_gender = State()
    get_location = State()
    view_form = State()


user_info_dialog = Dialog(
    Window(
        MSG_FIRST,
        Next(Const("✅ Продолжить"), id="return_to_age"),
        state=UserInfo.preview,
    ),
    Window(
        MSG_GET_AGE,
        BTN_GET_AGE,
        Next(
            Const("✅ Продолжить"),
            id="return_to_gender",
            on_click=UserInfoHandler.save_age,
        ),
        getter=CalkStepsHandler(StepsAddInfo).get_steps,
        state=UserInfo.get_age,
    ),
    Window(
        MSG_GET_GENDER,
        BTN_GET_USER_GENDER,
        Next(
            Const("✅ Продолжить"),
            id="return_to_location",
            on_click=UserInfoHandler.save_gender,
        ),
        getter=CalkStepsHandler(StepsAddInfo).get_steps,
        state=UserInfo.get_gender,
    ),
    Window(
        MSG_GET_LOCATION,
        BTN_GETTING_LOCATION,
        MessageInput(
            UserInfoHandler.save_location, content_types=[ContentType.LOCATION]
        ),
        markup_factory=ReplyKeyboardFactory(
            input_field_placeholder=PLACEHOLDER_FOR_MURK_UP,
            resize_keyboard=True,
            one_time_keyboard=False,
        ),
        getter=CalkStepsHandler(StepsAddInfo).get_steps,
        state=UserInfo.get_location,
    ),
    Window(
        MSG_VIEW_FORM,
        Start(
            Const("🏠 На главную"),
            id="go_to_home",
            state=MainDialog.main,
            on_click=UserInfoHandler.create_user,
        ),
        SwitchTo(
            Const("🔙 Заполнить заново"), id="return_to_age", state=UserInfo.get_age
        ),
        getter=UserInfoHandler.get_view_form_user,
        state=UserInfo.view_form,
    ),
)

from enum import Enum

from aiogram.filters.state import State, StatesGroup
from aiogram.types import ContentType
from aiogram_dialog import Dialog, Window
from aiogram_dialog.widgets.input import MessageInput, TextInput
from aiogram_dialog.widgets.kbd import Button, Calendar, Next
from aiogram_dialog.widgets.markup.reply_keyboard import ReplyKeyboardFactory
from aiogram_dialog.widgets.text import Const

from src.adapters.getting_executor import GettingExecutor
from src.handlers.calc_steps import CalkStepsHandler
from src.handlers.user_info import UserInfoHandler
from src.keyboards.main import BTN_GO_TO_HOME
from src.keyboards.user_info import BTN_GETTING_LOCATION, KEYBOARD_GETTING_INTERESTS
from src.messages.base import PLACEHOLDER_FOR_MURK_UP
from src.messages.user_info import (
    MSG_FIRST,
    MSG_GET_AGE,
    MSG_GET_BIO,
    MSG_GET_INTERESTS,
    MSG_GET_LOCATION,
    MSG_VIEW_FORM,
)
from src.validators.user_bio import BioValidator


class StepsAddInfo(Enum):
    BIRTH_DAY = "birth_day"
    LOCATION = "location"
    INTERESTS = "interests"
    BIO = "bio"


class UserInfo(StatesGroup):
    preview = State()
    get_birth_day = State()
    get_location = State()
    get_interests = State()
    get_bio = State()
    view_form = State()


user_info_dialog = Dialog(
    Window(
        MSG_FIRST,
        Next(Const("✅ Продолжить"), id="return_to_age"),
        state=UserInfo.preview,
    ),
    Window(
        MSG_GET_AGE,
        Calendar(id="calendar", on_click=UserInfoHandler.save_birth_day),
        getter=CalkStepsHandler(StepsAddInfo).get_steps,
        state=UserInfo.get_birth_day,
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
        MSG_GET_INTERESTS,
        KEYBOARD_GETTING_INTERESTS,
        Button(
            Const("✅ Продолжить"),
            id="return_to_bio",
            on_click=UserInfoHandler.save_interests,
        ),
        getter=GettingExecutor(
            CalkStepsHandler(StepsAddInfo).get_steps, UserInfoHandler.get_list_interests
        ).execute,
        state=UserInfo.get_interests,
    ),
    Window(
        MSG_GET_BIO,
        Next(Const("⏩ Пропустить"), id="skip_enter_bio"),
        TextInput(
            id="bio",
            type_factory=BioValidator.validate,
            on_success=UserInfoHandler.save_bio,
            on_error=BioValidator.error,
        ),
        getter=CalkStepsHandler(StepsAddInfo).get_steps,
        state=UserInfo.get_bio,
    ),
    Window(
        MSG_VIEW_FORM,
        BTN_GO_TO_HOME,
        getter=UserInfoHandler.get_view_form_user,
        state=UserInfo.view_form,
    ),
)

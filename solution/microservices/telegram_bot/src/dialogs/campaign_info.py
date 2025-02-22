import operator
from enum import Enum

from aiogram.filters.state import State, StatesGroup
from aiogram_dialog import Dialog, Window
from aiogram_dialog.widgets.input import TextInput
from aiogram_dialog.widgets.kbd import Cancel, Multiselect, Next, Row, SwitchTo
from aiogram_dialog.widgets.text import Const, Format

from src.buttons.user_info import BTN_GET_USER_GENDER
from src.handlers.calc_steps import CalkStepsHandler
from src.handlers.campaign_info import CampaignInfoHandler
from src.messages.campaign_info import (
    MSG_FIRST,
    MSG_GENERATE_TEXT,
    MSG_GENERATED_TEXT,
    MSG_GET_AD_TEXT,
    MSG_GET_AD_TITLE,
    MSG_GET_CLICKS_LIMIT,
    MSG_GET_COST_PER_CLICK,
    MSG_GET_COST_PER_VIEW,
    MSG_GET_END_DATE,
    MSG_GET_START_DATE,
    MSG_GET_TARGETING_AGE_FROM,
    MSG_GET_TARGETING_AGE_TO,
    MSG_GET_TARGETING_GENDER,
    MSG_GET_TARGETING_LOCATION,
    MSG_GET_VIEW_LIMIT,
    MSG_VIEW_FORM,
)
from src.validators.float_num import FloatNumValidator
from src.validators.uint_num import UIntNumValidator


class StepsAddInfo(Enum):
    START_DATE = "start_date"
    END_DATE = "end_date"
    VIEW_LIMIT = "view_limit"
    CLICKS_LIMIT = "clicks_limit"
    COST_PER_VIEW = "cost_per_view"
    COST_PER_CLICK = "cost_per_click"
    TARGETING_AGE_FROM = "targeting_age_from"
    TARGETING_AGE_TO = "targeting_age_to"
    TARGETING_GENDER = "targeting_gender"
    TARGETING_LOCATION = "targeting_location"
    AD_TITLE = "ad_title"
    AD_TEXT = "ad_text"


class CampaignInfoDialog(StatesGroup):
    home = State()
    get_start_date = State()
    get_end_date = State()
    get_view_limit = State()
    get_clicks_limit = State()
    get_cost_per_view = State()
    get_cost_per_click = State()
    get_targeting_age_from = State()
    get_targeting_age_to = State()
    get_targeting_gender = State()
    get_targeting_location = State()
    get_ad_title = State()
    get_ad_text = State()
    view_form = State()
    generate_text = State()
    generated_text = State()


campaign_info_dialog = Dialog(
    Window(
        MSG_FIRST,
        Next(Const("✅ Продолжить"), id="return_to_name"),
        state=CampaignInfoDialog.home,
    ),
    Window(
        MSG_GET_START_DATE,
        TextInput(
            id="get_start_date",
            on_error=UIntNumValidator.error,
            type_factory=UIntNumValidator.validate,
            on_success=CampaignInfoHandler.save_start_date,
        ),
        state=CampaignInfoDialog.get_start_date,
        getter=CalkStepsHandler(StepsAddInfo, step=1).get_steps,
    ),
    Window(
        MSG_GET_END_DATE,
        TextInput(
            id="get_end_date",
            on_error=UIntNumValidator.error,
            type_factory=UIntNumValidator.validate,
            on_success=CampaignInfoHandler.save_end_date,
        ),
        state=CampaignInfoDialog.get_end_date,
        getter=CalkStepsHandler(StepsAddInfo, step=2).get_steps,
    ),
    Window(
        MSG_GET_VIEW_LIMIT,
        TextInput(
            id="get_view_limit",
            on_error=UIntNumValidator.error,
            type_factory=UIntNumValidator.validate,
            on_success=CampaignInfoHandler.save_impressions_limit,
        ),
        state=CampaignInfoDialog.get_view_limit,
        getter=CalkStepsHandler(StepsAddInfo, step=3).get_steps,
    ),
    Window(
        MSG_GET_CLICKS_LIMIT,
        TextInput(
            id="get_clicks_limit",
            on_error=UIntNumValidator.error,
            type_factory=UIntNumValidator.validate,
            on_success=CampaignInfoHandler.save_clicks_limit,
        ),
        state=CampaignInfoDialog.get_clicks_limit,
        getter=CalkStepsHandler(StepsAddInfo, step=4).get_steps,
    ),
    Window(
        MSG_GET_COST_PER_VIEW,
        TextInput(
            id="get_cost_per_view",
            on_error=FloatNumValidator.error,
            type_factory=FloatNumValidator.validate,
            on_success=CampaignInfoHandler.save_cost_per_impression,
        ),
        state=CampaignInfoDialog.get_cost_per_view,
        getter=CalkStepsHandler(StepsAddInfo, step=5).get_steps,
    ),
    Window(
        MSG_GET_COST_PER_CLICK,
        TextInput(
            id="get_cost_per_click",
            on_error=FloatNumValidator.error,
            type_factory=FloatNumValidator.validate,
            on_success=CampaignInfoHandler.save_cost_per_click,
        ),
        state=CampaignInfoDialog.get_cost_per_click,
        getter=CalkStepsHandler(StepsAddInfo, step=6).get_steps,
    ),
    Window(
        MSG_GET_TARGETING_AGE_FROM,
        TextInput(
            id="get_targeting_age_from",
            on_error=UIntNumValidator.error,
            type_factory=UIntNumValidator.validate,
            on_success=CampaignInfoHandler.save_targeting_age_from,
        ),
        Next(Const("⏩ Пропустить")),
        state=CampaignInfoDialog.get_targeting_age_from,
        getter=CalkStepsHandler(StepsAddInfo, step=7).get_steps,
    ),
    Window(
        MSG_GET_TARGETING_AGE_TO,
        TextInput(
            id="get_targeting_age_to",
            on_error=UIntNumValidator.error,
            type_factory=UIntNumValidator.validate,
            on_success=CampaignInfoHandler.save_targeting_age_to,
        ),
        Next(Const("⏩ Пропустить")),
        state=CampaignInfoDialog.get_targeting_age_to,
        getter=CalkStepsHandler(StepsAddInfo, step=8).get_steps,
    ),
    Window(
        MSG_GET_TARGETING_GENDER,
        BTN_GET_USER_GENDER,
        Next(
            Const("✅ Продолжить"),
            id="get_next_user_gender",
            on_click=CampaignInfoHandler.save_targeting_gender,
        ),
        Next(Const("⏩ Пропустить")),
        getter=CalkStepsHandler(StepsAddInfo, step=9).get_steps,
        state=CampaignInfoDialog.get_targeting_gender,
    ),
    Window(
        MSG_GET_TARGETING_LOCATION,
        TextInput(
            id="get_targeting_location",
            on_success=CampaignInfoHandler.save_targeting_location,
        ),
        Next(Const("⏩ Пропустить")),
        getter=CalkStepsHandler(StepsAddInfo, step=10).get_steps,
        state=CampaignInfoDialog.get_targeting_location,
    ),
    Window(
        MSG_GET_AD_TITLE,
        TextInput(
            id="get_ad_title",
            on_success=CampaignInfoHandler.save_ad_title,
        ),
        getter=CalkStepsHandler(StepsAddInfo, step=11).get_steps,
        state=CampaignInfoDialog.get_ad_title,
    ),
    Window(
        MSG_GET_AD_TEXT,
        TextInput(
            id="get_ad_text",
            on_success=CampaignInfoHandler.save_ad_text,
        ),
        getter=CalkStepsHandler(StepsAddInfo, step=12).get_steps,
        state=CampaignInfoDialog.get_ad_text,
    ),
    Window(
        MSG_VIEW_FORM,
        Next(
            Const("✅ Продолжить"),
            on_click=CampaignInfoHandler.create_campaign,
        ),
        SwitchTo(
            Const("🔙 Заполнить заново"),
            id="return_to_name",
            state=CampaignInfoDialog.get_start_date,
        ),
        Cancel(
            Const("🏠 На главную"),
        ),
        getter=CampaignInfoHandler.get_view_form_campaign,
        state=CampaignInfoDialog.view_form,
    ),
    Window(
        MSG_GENERATE_TEXT,
        Row(
            Multiselect(
                Format("✓ {item[0]}"),
                Format("{item[0]}"),
                id="getting_generate_text",
                item_id_getter=operator.itemgetter(1),
                items=[("Заголовок", "TITLE"), ("Текст", "TEXT")],
            ),
        ),
        Next(Const("🤖 Сгенерировать"), on_click=CampaignInfoHandler.generate_text),
        Cancel(
            Const("⏩ Пропустить"),
        ),
        state=CampaignInfoDialog.generate_text,
    ),
    Window(
        MSG_GENERATED_TEXT,
        Cancel(
            Const("⏩ Пропустить и выйти"),
        ),
        Cancel(
            Const("✅ Принять и выйти"),
        ),
        Next(
            Const("🤖 Сгенерировать ещё раз"),
            on_click=CampaignInfoHandler.generate_text,
        ),
        state=CampaignInfoDialog.generated_text,
        getter=CampaignInfoHandler.get_generated_text,
    ),
)

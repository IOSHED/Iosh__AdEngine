from aiogram.filters.state import State, StatesGroup
from aiogram_dialog import Dialog, Window
from aiogram_dialog.widgets.kbd import Button, Cancel, Next, Row, Start, SwitchTo
from aiogram_dialog.widgets.text import Const, Format

from src.dialogs.campaign_info import CampaignInfoDialog
from src.handlers.advertiser import AdvertiserHandler
from src.keyboards.moderate_words import BTN_MANAGE_WINDOW
from src.messages.advertiser import MSG_ADVERTISER, MSG_STATS_ADVERTISER
from src.messages.campaign_info import MSG_STATS_CAMPAIGN, MSG_VIEW_FORM


class AdvertiserDialog(StatesGroup):
    home = State()
    get_my_campaigns = State()
    create_campaign = State()
    get_stats = State()
    get_stats_advertiser = State()


advertiser_dialog = Dialog(
    Window(
        MSG_ADVERTISER,
        Start(
            Const("➕ Создать кампанию"),
            id="go_to_create_campaign",
            state=CampaignInfoDialog.home,
        ),
        SwitchTo(
            Const("📊 Статистика"),
            id="go_to_stats_advertiser",
            state=AdvertiserDialog.get_stats_advertiser,
        ),
        SwitchTo(
            Const("📝 Мои кампании"),
            id="go_to_my_campaigns",
            state=AdvertiserDialog.get_my_campaigns,
        ),
        Cancel(Const("🏠 На главную")),
        state=AdvertiserDialog.home,
    ),
    Window(
        MSG_VIEW_FORM,
        # Button(Const("🖋️ Редактировать"), id="edit_campaign"),
        Button(
            Const("🗑️ Удалить"),
            id="delete_campaign",
            on_click=AdvertiserHandler.delete_campaign,
        ),
        Next(
            Const("📊 Статистика"),
        ),
        Row(
            Button(
                Const("<"),
                id="go_back",
                on_click=AdvertiserHandler.go_back_campaign,
            ),
            Button(
                Format("{num_campaign:g}"),
                id="go_num",
            ),
            Button(
                Const(">"),
                id="go_next",
                on_click=AdvertiserHandler.go_next_campaign,
            ),
        ),
        BTN_MANAGE_WINDOW(AdvertiserDialog.home),
        state=AdvertiserDialog.get_my_campaigns,
        getter=AdvertiserHandler.get_campaign_data,
    ),
    Window(
        MSG_STATS_CAMPAIGN,
        BTN_MANAGE_WINDOW(AdvertiserDialog.get_my_campaigns),
        getter=AdvertiserHandler.get_stats,
        state=AdvertiserDialog.get_stats,
    ),
    Window(
        MSG_STATS_ADVERTISER,
        BTN_MANAGE_WINDOW(AdvertiserDialog.home),
        getter=AdvertiserHandler.get_stats_advertiser,
        state=AdvertiserDialog.get_stats_advertiser,
    ),
)

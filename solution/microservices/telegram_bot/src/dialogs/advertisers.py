from aiogram.filters.state import State, StatesGroup
from aiogram_dialog import Dialog, Window
from aiogram_dialog.widgets.kbd import Start, SwitchTo
from aiogram_dialog.widgets.text import Const

from src.dialogs.campaign_info import CampaignInfoDialog
from src.messages.advertiser import MSG_ADVERTISER


class AdvertiserDialog(StatesGroup):
    home = State()
    get_my_campaigns = State()
    create_campaign = State()
    get_stats = State()


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
            id="go_to_stats",
            state=AdvertiserDialog.get_stats,
        ),
        SwitchTo(
            Const("📝 Мои кампании"),
            id="go_to_my_campaigns",
            state=AdvertiserDialog.get_my_campaigns,
        ),
        state=AdvertiserDialog.home,
    ),
)

from aiogram.filters.state import State, StatesGroup
from aiogram_dialog import (
    Dialog,
    Window,
)
from aiogram_dialog.widgets.kbd import Button, Start, SwitchTo
from aiogram_dialog.widgets.text import Const

from src.dialogs.moderate_words import ModerateDialog
from src.handlers.advertiser_check import AdvertiserCheckHandler
from src.messages.main import MSG_MAIN


class MainDialog(StatesGroup):
    main = State()
    view_ads = State()
    advertiser = State()


main_dialog = Dialog(
    Window(
        MSG_MAIN,
        # SwitchTo(Const("👤 Мой профиль"), id="go_to_user_info", state=MainDialog.main),
        SwitchTo(
            Const("📽️ Смотреть рекламу"), id="go_to_view_ads", state=MainDialog.view_ads
        ),
        Button(
            Const("⚒️ Рекламодателям"),
            id="go_to_advertiser",
            on_click=AdvertiserCheckHandler.check_exists,
        ),
        Start(Const("⛔ Администрации"), id="go_to_admin", state=ModerateDialog.home),
        state=MainDialog.main,
    ),
)

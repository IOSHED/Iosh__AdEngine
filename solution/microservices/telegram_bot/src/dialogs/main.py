from aiogram.filters.state import State, StatesGroup
from aiogram_dialog import (
    Dialog,
    Window,
)
from aiogram_dialog.widgets.kbd import Back, Start, SwitchTo
from aiogram_dialog.widgets.text import Const

from src.dialogs.moderate_words import ModerateWordsDialog
from src.handlers.time import TimeHandler
from src.keyboards.main import BTN_COUNTER_TIME_ADVANCE
from src.messages.main import MSG_ADMIN_PANEL, MSG_MAIN


class MainDialog(StatesGroup):
    main = State()
    view_ads = State()
    advertiser = State()
    admin = State()
    set_time_advance = State()


main_dialog = Dialog(
    Window(
        MSG_MAIN,
        # SwitchTo(Const("👤 Мой профиль"), id="go_to_user_info", state=MainDialog.main),
        SwitchTo(
            Const("📽️ Смотреть рекламу"), id="go_to_view_ads", state=MainDialog.view_ads
        ),
        SwitchTo(
            Const("⚒️ Рекламодателям"),
            id="go_to_advertiser",
            state=MainDialog.advertiser,
        ),
        SwitchTo(Const("⛔ Администрации"), id="go_to_admin", state=MainDialog.admin),
        state=MainDialog.main,
    ),
    Window(
        MSG_ADMIN_PANEL,
        SwitchTo(
            Const("🕐 Промотать время"),
            id="go_to_time_advance",
            state=MainDialog.set_time_advance,
        ),
        Start(
            Const("🤬 Модерация текста"),
            id="go_to_moderate",
            state=ModerateWordsDialog.main,
        ),
        SwitchTo(Const("🏠 На главную"), id="go_to_home", state=MainDialog.main),
        state=MainDialog.admin,
    ),
    Window(
        Const("🕐 Промотать время до"),
        BTN_COUNTER_TIME_ADVANCE,
        Back(
            Const("✅ Подтвердить"),
            id="confirm_time_advance",
            on_click=TimeHandler.set_time_advance,
        ),
        SwitchTo(Const("🏠 На главную"), id="go_to_home", state=MainDialog.main),
        state=MainDialog.set_time_advance,
    ),
)

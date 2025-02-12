from aiogram_dialog.widgets.kbd import Start
from aiogram_dialog.widgets.text import Const

from src.dialogs.main import MainDialog

BTN_GO_TO_HOME = Start(Const("🏠 На главную"), id="go_to_home", state=MainDialog.main)

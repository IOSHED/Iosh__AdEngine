from aiogram_dialog.widgets.text import Const, Format, Multi

from src.messages.base import PART_STAGES

MSG_FIRST = Const("👏<b>Здравствуй</b>!👏\n\n Для начала заполним некоторые данные.")

MSG_GET_NAME = Multi(
    PART_STAGES,
    Const("Введи 🎉<b>имя</b>🖼️, под которым ты будешь рекламировать:"),
)

MSG_VIEW_FORM = Multi(
    Const("Готова твоя 📋<b>Анкета Рекламодателя</b>📋:\n"),
    Format("\t1️⃣ Имя: <b>{name}</b>"),
)

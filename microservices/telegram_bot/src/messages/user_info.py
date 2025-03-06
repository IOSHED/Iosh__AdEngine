from aiogram_dialog.widgets.text import Const, Format, Multi

from src.messages.base import PART_STAGES

MSG_FIRST = Const(
    "👏<b>Здравствуй</b>!👏\n\n Для начала заполним некоторые данные о тебе."
)

MSG_GET_AGE = Multi(
    PART_STAGES,
    Const("Введи свой 🎉<b>Сколько тебе лет</b>✨:"),
)

MSG_GET_LOCATION = Multi(
    PART_STAGES,
    Const("Поделись своей 🌎<b>Локацией</b>🌎:"),
)

MSG_GET_GENDER = Multi(
    PART_STAGES,
    Const("Выбери свой 🚹<b>Пол</b>🚺:"),
)

MSG_VIEW_FORM = Multi(
    Const("Готова твоя 📋<b>Анкета</b>📋:\n"),
    Format("\t1️⃣ Тебе <b>{age}</b> лет"),
    Format("\t3️⃣ Ты <b>{gender}</b>"),
    Format("\t2️⃣ Ты находишься в <b>{city}</b>, <b>{country}</b>"),
)

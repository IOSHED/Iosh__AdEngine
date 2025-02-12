from aiogram_dialog.widgets.text import Const, Format, Multi

from src.messages.base import PART_STAGES

MSG_FIRST = Const(
    "👏<b>Здравствуй</b>!👏\n\n Для начала заполни некоторые данные о себе."
)

MSG_GET_AGE = Multi(
    PART_STAGES,
    Const("Введи свой 🎉<b>День Рождения</b>✨:"),
)

MSG_GET_LOCATION = Multi(
    PART_STAGES,
    Const("Поделись своей 🌎<b>Локацией</b>🌎:"),
)

MSG_GET_INTERESTS = Multi(
    PART_STAGES,
    Const("Выбери, что тебе 🥋<b>Интересно</b>🤿:"),
)

MSG_GET_BIO = Multi(
    PART_STAGES,
    Const("Введи своё 📜<b>Bio</b>📜, если хочешь:"),
)

MSG_VIEW_FORM = Multi(
    Const("Готова твоя 📋<b>Анкета</b>📋:\n"),
    Format("\t1️⃣ Твой день рождение <b>{birth_day}</b>"),
    Format("\t2️⃣ Ты находишься в <b>{city}</b>, <b>{country}</b>"),
    Format("\t3️⃣ Ты любишь <b>{str_interests}</b>"),
    Format("\t4️⃣ Твоё <code>bio</code>: <blockquote expandable>{bio}</blockquote>"),
)

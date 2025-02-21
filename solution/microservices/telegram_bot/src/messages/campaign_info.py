from aiogram_dialog.widgets.text import Const, Format, Multi
from magic_filter import F

from src.messages.base import PART_STAGES

MSG_FIRST = Const(
    "👏<b>Здравствуй</b>!👏\n\n Заполним некоторые данные о рекламной кампании."
)

MSG_GET_START_DATE = Multi(
    PART_STAGES,
    Const("Введите когда 🕑<b>начнётся</b>🕑 рекламная кампания:"),
)

MSG_GET_END_DATE = Multi(
    PART_STAGES,
    Const("Введите когда 🕑<b>закончится</b>🕑 рекламная кампания:"),
)

MSG_GET_VIEW_LIMIT = Multi(
    PART_STAGES,
    Const("Введите 🧱<b>Лимит просмотров</b>🧱:"),
)

MSG_GET_CLICKS_LIMIT = Multi(
    PART_STAGES,
    Const("Введите 🧱<b>Лимит кликов</b>🧱:"),
)

MSG_GET_COST_PER_VIEW = Multi(
    PART_STAGES,
    Const("Введите 💵<b>Стоимость просмотра</b>💵:"),
)

MSG_GET_COST_PER_CLICK = Multi(
    PART_STAGES,
    Const("Введите 💵<b>Стоимость клика</b>💵:"),
)

MSG_GET_TARGETING_AGE_FROM = Multi(
    PART_STAGES,
    Const("Введите от какого 🧑<b>возраста</b>🧑 будут просматривать рекламу:"),
)

MSG_GET_TARGETING_AGE_TO = Multi(
    PART_STAGES,
    Const("Введите до какого 🧑<b>возраста</b>🧑 будут просматривать рекламу:"),
)

MSG_GET_TARGETING_GENDER = Multi(
    PART_STAGES,
    Const(
        "Выберите 🚻<b>Пол</b>🚺 пользователей, которые будут просматривать рекламу:"
    ),
)

MSG_GET_TARGETING_LOCATION = Multi(
    PART_STAGES,
    Const("Введите 🌎<b>Локацию</b>🌎, где будут просматривать рекламу:"),
)

MSG_GET_AD_TITLE = Multi(
    PART_STAGES,
    Const(
        "<i>Если не уверены, введите ключевые слова для генерации заголовка gpt ботом.</i>\n"
    ),
    Const("Введите ⌨️<b>Название</b>⌨️ рекламной кампании:"),
)

MSG_GET_AD_TEXT = Multi(
    PART_STAGES,
    Const(
        "<i>Если не уверены, введите ключевые слова для генерации текста gpt ботом.</i>\n"
    ),
    Const("Введите 📝<b>Текст</b>📝 рекламной кампании:"),
)

MSG_VIEW_FORM = Multi(
    Const("Твоя 📋<b>Рекламная кампания</b>📋:\n"),
    Format("Заголовок: <blockquote expandable>{ad_title}</blockquote>\n"),
    Format("Текст: <blockquote expandable>{ad_text}</blockquote>\n"),
    Format("\t1️⃣ Дата действия с <b>{start_date}</b> до <b>{end_date}</b>\n"),
    Format("\t2️⃣ Лимит просмотров: <b>{impressions_limit}</b>"),
    Format("\t3️⃣ Лимит кликов: <b>{clicks_limit}</b>\n"),
    Format("\t4️⃣ Стоимость просмотра: <b>{cost_per_impression}</b>"),
    Format("\t5️⃣ Стоимость клика: <b>{cost_per_click}\n</b>"),
    Multi(
        Const("Твой 👀<b>Таргетинг</b>👀. Рекламу смотрят клиенты:"),
        Multi(
            Format("\t🟢 Возрастом", when=F["is_targeting_age"]),
            Format(" от <b>{age_from}</b>лет", when=F["is_targeting_age_from"]),
            Format(" до <b>{age_to}</b>лет", when=F["is_targeting_age_to"]),
            sep="",
        ),
        Format("\t🟢 {gender}", when=F["is_targeting_gender"]),
        Format("\t🟢 В локации - {location}", when=F["is_targeting_location"]),
        when=F["is_targeting"],
    ),
)

MSG_STATS_CAMPAIGN = Multi(
    Const("Статистика по твой рекламной кампании:\n"),
    Format("🟠Просмотры: <b>{impressions_count}</b>"),
    Format("🟠Клики: <b>{clicks_count}</b>"),
    Format("🔶Конверсия: <b>{conversion}</b>\n"),
    Format("🟠Затрачено на просмотры: <b>{spent_impressions}</b>"),
    Format("🟠Затрачено на клики: <b>{spent_clicks}</b>"),
    Format("🔶Затрачено всего: <b>{spent_total}</b>\n"),
)

MSG_GENERATE_TEXT = Multi(Const("Сгенерировать ли заголовок, ли текст для рекламы?"))

MSG_GENERATED_TEXT = Multi(
    Const("Сгенерировано!\n"),
    Format("\t1️⃣ Заголовок: <blockquote expandable>{ad_title}</blockquote>\n"),
    Format("\t2️⃣ Текст: <blockquote expandable>{ad_text}</blockquote>\n"),
)

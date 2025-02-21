from aiogram_dialog.widgets.text import Const, Format, Multi

MSG_ADVERTISER = Multi(
    Const("⚒️<b>Рекламодатель</b>!⚒️\n\n Вот куда ты можешь перейти:"),
    Multi(
        Const(
            "1️⃣ <b>Создать рекламную кампанию</b>: там ты сможешь создать свою рекламу"
        ),
        Const(
            "2️⃣ <b>Статистика</b>: можешь любоваться цифрами твоих рекламных кампаний"
        ),
        Const(
            "3️⃣ <b>МОи рекламные кампании</b>: можешь просмотреть, отредактировать, удалить кампанию"
        ),
        sep="\n",
    ),
)


MSG_STATS_ADVERTISER = Multi(
    Const("Вот твоя полная статистика за всё время:\n"),
    Format("🔹Просмотры: <b>{impressions_count}</b>"),
    Format("🔹Клики: <b>{clicks_count}</b>"),
    Format("🔵Конверсия: <b>{conversion}</b>\n"),
    Format("🔹Затрачено на просмотры: <b>{spent_impressions}</b>"),
    Format("🔹Затрачено на клики: <b>{spent_clicks}</b>"),
    Format("🔵Затрачено всего: <b>{spent_total}</b>\n"),
)

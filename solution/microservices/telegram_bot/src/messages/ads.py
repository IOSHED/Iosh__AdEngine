from aiogram_dialog.widgets.text import Format, Multi

MSG_ADS = Multi(
    Format("{ad_title}\n"),
    Format("{ad_text}\n"),
    Format("`{advertiser_id}`"),
)

from typing import List

from aiogram_dialog.widgets.text import Format

PART_STAGES = Format("🚩{num_finish_steps}/{all_steps}\n")

INTERESTS: List[str] = [
    ("Литература 📖", "Литература"),
    ("Спорт 🏸", "Спорт"),
    ("Экстрим ⚔️", "Экстрим"),
    ("Музыка 🎶", "Музыка"),
    ("Кино 🎥", "Кино"),
    ("Игры 🎮", "Игры"),
    ("Творчество 🎨", "Творчество"),
    ("Другое 🌱", "Другое"),
]


PLACEHOLDER_FOR_MURK_UP = Format("Нажми на кнопку ниже {event.from_user.username} )")

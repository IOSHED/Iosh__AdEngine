import asyncio
import logging
from typing import Tuple

from aiogram import Bot, Dispatcher
from aiogram.client.default import DefaultBotProperties
from aiogram.enums.parse_mode import ParseMode
from aiogram.fsm.storage.base import DefaultKeyBuilder
from aiogram.fsm.storage.redis import RedisStorage
from aiogram_dialog import setup_dialogs

from src.config import SETTINGS
from src.logger import setup_logger


async def main() -> Tuple[Dispatcher, Bot]:
    await setup_logger()
    logging.info("Starting bot")

    from src.dialogs.main import main_dialog
    from src.dialogs.user_info import user_info_dialog
    from src.entry_point import start_router
    from src.infra.redis_client import RedisClient

    storage = RedisStorage.from_url(
        RedisClient.get_url(),
        key_builder=DefaultKeyBuilder(with_destiny=True),
    )
    # await storage.redis.flushdb()  # TODO: delete to prod

    logging.info("Initialized Redis storage")

    bot = Bot(
        token=SETTINGS.bot.token,
        default=DefaultBotProperties(parse_mode=ParseMode.HTML),
    )

    dp = Dispatcher(storage=storage)

    dp.include_routers(
        start_router,
        main_dialog,
        user_info_dialog,
    )

    setup_dialogs(dp)

    logging.info("Initialized bot")
    return dp, bot


if __name__ == "__main__":
    dp, bot = asyncio.run(main())
    dp.run_polling(bot)

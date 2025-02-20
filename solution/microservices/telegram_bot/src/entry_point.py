from aiogram import Router
from aiogram.filters import CommandStart
from aiogram.types import Message
from aiogram_dialog import (
    DialogManager,
    StartMode,
)

from src.dialogs.main import MainDialog
from src.dialogs.user_info import UserInfo
from src.services.ad_engine.bundle_utils import generate_uuid_from_id
from src.services.ad_engine.client import ClientService

start_router = Router()


@start_router.message(CommandStart())
async def start(message: Message, dialog_manager: DialogManager) -> None:
    """
    Handle /start command and initialize dialog flow based on user existence.

    This handler checks if the user already exists in the travel service database.
    If they exist, it starts the main dialog flow. Otherwise, it redirects to
    the user info preview flow for registration.

    Args:
        message (Message): Incoming message object containing user information
        dialog_manager (DialogManager): Manager instance to control dialog states

    Returns:
        None: This function doesn't return anything but starts a dialog flow
    """
    state = (
        MainDialog.main
        if await ClientService.get_client_by_id(
            generate_uuid_from_id(message.from_user.id)
        )
        is not None
        else UserInfo.get_age
    )
    await dialog_manager.start(state, mode=StartMode.NORMAL)

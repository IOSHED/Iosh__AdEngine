import logging
from datetime import date
from typing import Any, Dict

from aiogram.types import CallbackQuery, Message
from aiogram_dialog import DialogManager
from aiogram_dialog.widgets.input import MessageInput
from aiogram_dialog.widgets.kbd import Calendar

from src.messages.base import INTERESTS
from src.services.travel_service.schemas import UserCreateSchema
from src.services.travel_service.user import TravelServiceUser


class UserInfoHandler:
    """Handler class for managing user information in a dialog-based system.

    This class provides methods to handle user data collection and management through
    a dialog interface, including interests, profile information, location, and bio.
    """

    @classmethod
    async def get_list_interests(
        cls,
        _dialog_manager: DialogManager,
        **_kwargs,
    ) -> Dict[str, Any]:
        """Retrieve the list of available interests.

        Args:
            _dialog_manager: Dialog manager instance (unused)
            **_kwargs: Additional keyword arguments

        Returns:
            Dict containing the list of interests
        """
        return {
            "list_interests": INTERESTS,
        }

    @classmethod
    async def get_view_form_user(
        cls,
        dialog_manager: DialogManager,
        **_kwargs,
    ) -> Dict[str, Any]:
        """Generate a user profile view based on collected dialog data.

        Args:
            dialog_manager: Dialog manager containing user data
            **_kwargs: Additional keyword arguments

        Returns:
            Dict containing formatted user profile information
        """
        create_schema = UserCreateSchema(
            telegram_id=dialog_manager.event.from_user.id,
            birth_day=dialog_manager.dialog_data["birth_day"],
            latitude=dialog_manager.dialog_data["location"]["latitude"],
            longitude=dialog_manager.dialog_data["location"]["longitude"],
            interests=dialog_manager.dialog_data.get("interests", None),
            bio=dialog_manager.dialog_data.get("bio", None),
        )
        user_profile = await TravelServiceUser.creating_user(create_schema)
        str_interests = (
            ", ".join(user_profile.interests) if user_profile.interests else "ничего"
        )

        return {
            "birth_day": user_profile.birth_day,
            "city": user_profile.city,
            "country": user_profile.country,
            "str_interests": str_interests,
            "bio": user_profile.bio or "Отсутвует...",
        }

    @classmethod
    async def save_birth_day(
        cls,
        callback: CallbackQuery,
        _widget: Calendar,
        manager: DialogManager,
        selected_date: date,
    ) -> None:
        """Save user's birth date and advance dialog.

        Args:
            callback: Callback query from the calendar widget
            _widget: Calendar widget instance
            manager: Dialog manager instance
            selected_date: Selected date from calendar

        Returns:
            None
        """
        logging.debug(f"Parse birth_day: {selected_date}")
        manager.dialog_data["birth_day"] = selected_date.isoformat()
        await callback.answer(str(selected_date))
        await manager.next()

    @classmethod
    async def save_location(
        cls,
        message: Message,
        _message_input: MessageInput,
        manager: DialogManager,
    ) -> None:
        """Save user's location coordinates and advance dialog.

        Args:
            message: Message containing location data
            _message_input: Message input widget instance
            manager: Dialog manager instance

        Returns:
            None
        """
        logging.debug(f"Parse location: {message.location}")
        manager.dialog_data["location"] = {
            "latitude": message.location.latitude,
            "longitude": message.location.longitude,
        }
        await manager.next()

    @classmethod
    async def save_interests(
        cls,
        _callback: CallbackQuery,
        _widget: Any,
        manager: DialogManager,
    ) -> None:
        """Save user's selected interests and advance dialog.

        Args:
            _callback: Callback query from interest selection
            _widget: Widget instance
            manager: Dialog manager instance

        Returns:
            None
        """
        selected_ids = manager.find("getting_list_interests").get_checked()
        logging.debug(f"Parse interests: {selected_ids}")
        manager.dialog_data["interests"] = selected_ids
        await manager.next()

    @classmethod
    async def save_bio(
        cls,
        message: Message,
        _source: Any,
        manager: DialogManager,
        *_args,
        **_kwargs,
    ) -> None:
        """Save user's biography text and advance dialog.

        Args:
            message: Message containing bio text
            _source: Source widget instance
            manager: Dialog manager instance
            *_args: Additional positional arguments
            **_kwargs: Additional keyword arguments

        Returns:
            None
        """
        logging.debug(f"Parse bio: {message.text}")
        manager.dialog_data["bio"] = message.text
        await manager.next()

import logging
from typing import Any, Dict

from aiogram.types import CallbackQuery, Message
from aiogram_dialog import DialogManager
from aiogram_dialog.widgets.input import MessageInput

from src.services.ad_engine.bundle_utils import generate_uuid_from_id
from src.services.ad_engine.client import ClientService
from src.services.ad_engine.schemas.client import ClientProfileSchema
from src.services.location_service import LocationService


class UserInfoHandler:
    """Handler class for managing user information in a dialog-based system.

    This class provides methods to handle user data collection and management through
    a dialog interface, including interests, profile information, location, and bio.
    """

    @classmethod
    async def create_user(
        cls,
        callback: CallbackQuery,
        _widget: Any,
        manager: DialogManager,
    ) -> None:
        try:
            create_user = ClientProfileSchema(
                client_id=str(generate_uuid_from_id(callback.from_user.id)),
                login=f"{callback.from_user.first_name} {callback.from_user.last_name}",
                location=manager.dialog_data["location"]["city"],
                gender=manager.dialog_data["gender"],
                age=manager.dialog_data["age"],
            )

            logging.debug(f"Create user: {create_user}")

            await ClientService.create_client(create_user)

        except Exception as e:
            logging.error(f"Error creating user: {e}")
            await manager.back()
            callback.answer(
                "❌ Произошла ошибка при создании пользователя, попробуйте позже..."
            )

    @classmethod
    async def get_view_form_user(
        cls,
        dialog_manager: DialogManager,
        **_kwargs,
    ) -> Dict[str, Any]:
        city, country = await LocationService.get_city_country(
            dialog_manager.dialog_data["location"]["latitude"],
            dialog_manager.dialog_data["location"]["longitude"],
        )
        dialog_manager.dialog_data["location"]["city"] = city
        dialog_manager.dialog_data["location"]["country"] = country
        return {
            "age": dialog_manager.dialog_data["age"],
            "gender": "🚹 Мужчина"
            if dialog_manager.dialog_data["gender"] == "MALE"
            else "🚺 Женщина",
            "city": city,
            "country": country,
        }

    @classmethod
    async def save_age(
        cls,
        callback: CallbackQuery,
        manager: DialogManager,
    ) -> None:
        age = manager.find("counter_getting_age").get_value()
        logging.debug(f"Parse save_age: {age}")
        manager.dialog_data["age"] = age

    @classmethod
    async def save_location(
        cls,
        message: Message,
        _message_input: MessageInput,
        manager: DialogManager,
    ) -> None:
        logging.debug(f"Parse location: {message.location}")
        manager.dialog_data["location"] = {
            "latitude": message.location.latitude,
            "longitude": message.location.longitude,
        }
        await manager.next()

    @classmethod
    async def save_gender(
        cls,
        _callback: CallbackQuery,
        _widget: Any,
        manager: DialogManager,
    ) -> None:
        selected_gender = manager.find("getting_user_gender").get_checked()
        logging.debug(f"Parse interests: {selected_gender}")
        if selected_gender is None:
            await manager.back()
        else:
            manager.dialog_data["gender"] = selected_gender

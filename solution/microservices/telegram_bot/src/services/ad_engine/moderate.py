from typing import List

from src.services.ad_engine.schemas.client import ClientProfileSchema
from src.services.ad_engine.schemas.moderate import ModerateSchema
from src.services.http_serves_parser import HttpServesParser


class ModerateService(HttpServesParser):
    @classmethod
    async def set_moderate_settings(cls, settings: ModerateSchema) -> ModerateSchema:
        url = f"{cls._host_url}/moderate/config"
        try:
            response = await cls._make_request(
                method="POST", url=url, json_body=settings.model_dump()
            )
            return ModerateSchema(**response.json())

        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to create client: {str(e)}")

    @classmethod
    async def get_black_list_words(cls) -> List[str]:
        url = f"{cls._host_url}/moderate/list"

        try:
            response = await cls._make_request(method="GET", url=url)
            return response.json()

        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to get black list words: {str(e)}")

    @classmethod
    async def add_black_list_words(cls, words: List[str]) -> None:
        url = f"{cls._host_url}/moderate/list"

        try:
            _response = await cls._make_request(method="POST", url=url, json_body=words)
        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to add black list words: {str(e)}")

    @classmethod
    async def delete_black_list_words(cls, words: List[str]) -> None:
        url = f"{cls._host_url}/moderate/list"

        try:
            _response = await cls._make_request(
                method="DELETE", url=url, json_body=words
            )
        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to delte black list words: {str(e)}")

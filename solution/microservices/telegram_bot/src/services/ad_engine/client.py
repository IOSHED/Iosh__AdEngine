import uuid
from typing import Optional

from src.services.ad_engine.schemas.client import ClientProfileSchema
from src.services.http_serves_parser import HttpServesParser


class ClientService(HttpServesParser):
    @classmethod
    async def create_client(cls, client: ClientProfileSchema) -> ClientProfileSchema:
        url = f"{cls._base_url}/api/client/bulk"
        try:
            response = await cls._make_request(
                method="POST", url=url, json_body=[client.model_dump()]
            )

            client_data = response.json()
            return ClientProfileSchema(**client_data[0])

        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to create client: {str(e)}")

    @classmethod
    async def get_client_by_id(
        cls, client_id: uuid.UUID
    ) -> Optional[ClientProfileSchema]:
        url = f"{cls._base_url}/client/{client_id}"

        try:
            response = await cls._make_request(method="GET", url=url)

            if response is None:
                return None

            client_data = response.json()
            return ClientProfileSchema(**client_data)

        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to retrieve client: {str(e)}")

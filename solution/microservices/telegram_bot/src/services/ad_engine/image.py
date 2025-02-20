import io
import uuid
from ast import List
from typing import Optional

from src.services.http_serves_parser import HttpServesParser


class ImageService(HttpServesParser):
    _headers = {"Content-Type": "multipart/form-data"}

    @classmethod
    async def upload_images(
        cls,
        advertiser_id: uuid.UUID,
        campaign_id: uuid.UUID,
        images: List[(str, io.BytesIO, str)],
    ) -> Optional[str]:
        url = f"{cls._host_url}/advertisers/{advertiser_id}/campaigns/{campaign_id}/images"
        try:
            files = [
                ("file", (f"image_{file_name}.jpg", image_bytes, mime_type))
                for (file_name, image_bytes, mime_type) in images
            ]
            response = await cls._make_request(method="POST", url=url, files=files)
            if response is None:
                return None

            return "ok"

        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to upload images: {str(e)}")

    @classmethod
    async def get_file_names(
        cls, advertiser_id: uuid.UUID, campaign_id: uuid.UUID
    ) -> Optional[List[str]]:
        url = f"{cls._host_url}/advertisers/{advertiser_id}/campaigns/{campaign_id}/images"
        try:
            response = await cls._make_request(method="GET", url=url)

            if response is None:
                return None

            return response.json()
        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to get names images: {str(e)}")

    @classmethod
    async def get_image(
        cls, advertiser_id: uuid.UUID, campaign_id: uuid.UUID, file_name: str
    ) -> Optional[io.BytesIO]:
        url = f"{cls._host_url}/advertisers/{advertiser_id}/campaigns/{campaign_id}/images/{file_name}"
        try:
            response = await cls._make_request(method="GET", url=url)

            if response is None:
                return None

            return io.BytesIO(response.content)

        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to get names images: {str(e)}")

    @classmethod
    async def delete_file(
        cls, advertiser_id: uuid.UUID, campaign_id: uuid.UUID, file_name: str
    ) -> Optional[List[str]]:
        url = f"{cls._host_url}/advertisers/{advertiser_id}/campaigns/{campaign_id}/images/{file_name}"
        try:
            response = await cls._make_request(method="DELETE", url=url)

            if response is None:
                return None

            return response.json()
        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to get names images: {str(e)}")

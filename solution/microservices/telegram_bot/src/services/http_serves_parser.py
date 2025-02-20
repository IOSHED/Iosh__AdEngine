import logging
from typing import Optional

import httpx

from src.config import SETTINGS


class HttpServesParser:
    _host_url = SETTINGS.ad_engine.base_url

    _headers = {"Content-Type": "application/json"}

    @classmethod
    async def _make_request(
        cls, method: str, url: str, json_body: Optional[dict] = None
    ) -> Optional[httpx.Response]:
        """
        Helper method to make the HTTP request to avoid repetition.
        """
        async with httpx.AsyncClient() as client_session:
            try:
                if method == "POST":
                    response = await client_session.post(
                        url, json=json_body, headers=cls.headers
                    )
                elif method == "GET":
                    response = await client_session.get(url, headers=cls.headers)
                else:
                    raise ValueError(f"Unsupported HTTP method: {method}")

                if response.status_code == 404:
                    return None

                response.raise_for_status()
                return response

            except httpx.HTTPStatusError as e:
                cls._log_error(e)
                raise Exception(f"HTTP error occurred: {e.response.text}")

            except httpx.RequestError as e:
                cls._log_error(e)
                raise Exception(f"Request error occurred: {str(e)}")

            except Exception as e:
                cls._log_error(e)
                raise Exception(f"Unexpected error occurred: {str(e)}")

    @classmethod
    def _log_error(cls, error: Exception):
        """
        Centralized logging for errors.
        """
        logging.warning(f"Error: {str(error)}")

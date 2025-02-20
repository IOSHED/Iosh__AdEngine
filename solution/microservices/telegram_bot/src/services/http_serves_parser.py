import logging
from typing import Any, Dict, List, Optional, Tuple

import httpx
import requests

from src.config import SETTINGS


class HttpServesParser:
    _host_url = SETTINGS.ad_engine.base_url

    _headers = {"Content-Type": "application/json"}

    @classmethod
    async def _make_request(
        cls,
        method: str,
        url: str,
        json_body: Optional[dict] = None,
        params: Optional[Dict[str, Any]] = None,
        files: Optional[List[Tuple[Any]]] = None,
    ) -> Optional[httpx.Response]:
        """
        Helper method to make the HTTP request to avoid repetition.
        """
        async with httpx.AsyncClient() as client_session:
            if params:
                url = f"{url}?{httpx.URL(url).encode_query(params)}"

            try:
                if method == "POST":
                    response = await client_session.post(
                        url, json=json_body, headers=cls._headers, files=files
                    )

                elif method == "GET":
                    response = await client_session.get(url, headers=cls._headers)

                elif method == "DELETE":
                    response = requests.delete(
                        url,
                        data=json_body,
                        headers=cls._headers,
                    )
                    if response.status_code == 404:
                        return None
                    return response

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

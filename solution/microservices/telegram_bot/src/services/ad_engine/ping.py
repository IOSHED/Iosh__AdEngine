import time
import uuid

from src.services.http_serves_parser import HttpServesParser


class PingService(HttpServesParser):
    @classmethod
    async def get_ping(cls, client_id: uuid.UUID) -> float:
        url = f"{cls._host_url}/ping"

        start_time = time.time()

        try:
            response = await cls._make_request(method="GET", url=url)
            if response is None:
                raise Exception("not ping")

            end_time = time.time()

            response_time = end_time - start_time
            return response_time

        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed ping: {str(e)}")

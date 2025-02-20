from src.services.ad_engine.schemas.time import TimeAdvanceRequest, TimeAdvanceResponse
from src.services.http_serves_parser import HttpServesParser


class TimeService(HttpServesParser):
    @classmethod
    async def set(cls, time_advance: int) -> int:
        url = f"{cls._host_url}/time/advance"
        try:
            time_advance = TimeAdvanceRequest(current_date=time_advance)
            response = await cls._make_request(
                method="POST", url=url, json_body=time_advance.model_dump()
            )
            return TimeAdvanceResponse(**response.json()).current_date

        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to set time_advance: {str(e)}")

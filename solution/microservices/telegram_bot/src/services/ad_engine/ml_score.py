from src.services.ad_engine.schemas.ml_score import MlScoreRequest
from src.services.http_serves_parser import HttpServesParser


class MlScoreService(HttpServesParser):
    @classmethod
    async def create_ml_score(cls, ml_score: MlScoreRequest) -> None:
        url = f"{cls._base_url}/api/ml_score/bulk"
        try:
            _response = await cls._make_request(
                method="POST", url=url, json_body=ml_score.model_dump()
            )

        except Exception as e:
            cls._log_error(e)
            raise Exception(f"Failed to create ml-score: {str(e)}")

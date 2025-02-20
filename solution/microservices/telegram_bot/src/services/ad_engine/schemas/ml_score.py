import uuid

from pydantic import BaseModel, Field


class MlScoreRequest(BaseModel):
    client_id: uuid.UUID
    advertiser_id: uuid.UUID
    score: float = Field(..., ge=0)

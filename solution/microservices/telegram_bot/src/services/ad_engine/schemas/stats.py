from pydantic import BaseModel, Field


class StatDailyResponse(BaseModel):
    impressions_count: int = Field(..., ge=0)
    clicks_count: int = Field(..., ge=0)
    conversion: float
    spent_impressions: float
    spent_clicks: float
    spent_total: float
    date: int = Field(..., ge=0)


class StatResponse(BaseModel):
    impressions_count: int = Field(..., ge=0)
    clicks_count: int = Field(..., ge=0)
    conversion: float
    spent_impressions: float
    spent_clicks: float
    spent_total: float

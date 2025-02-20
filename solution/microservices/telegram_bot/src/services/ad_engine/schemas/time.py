from pydantic import BaseModel, Field


class TimeAdvanceRequest(BaseModel):
    current_date: int = Field(..., ge=0)


class TimeAdvanceResponse(BaseModel):
    current_date: int = Field(..., ge=0)

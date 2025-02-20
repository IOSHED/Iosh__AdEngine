from uuid import UUID

from pydantic import BaseModel, Field


class AdsSchema(BaseModel):
    ad_title: str = Field(..., description="Title of the advertisement")
    ad_text: str = Field(..., description="Main text content of the advertisement")
    advertiser_id: UUID = Field(
        ..., description="Unique identifier of the advertiser who created this ad"
    )
    ad_id: UUID = Field(..., description="Unique identifier for the campaign")

from typing import Optional
from uuid import UUID

from pydantic import BaseModel, Field, condecimal, conint


class TargetingCampaignSchema(BaseModel):
    age_from: Optional[int] = Field(
        None, ge=0, description="Minimum age of the target audience"
    )
    age_to: Optional[int] = Field(
        None, ge=0, description="Maximum age of the target audience"
    )
    gender: Optional[str] = Field(
        None, pattern=r"^(MALE|FEMALE)$", description="Gender targeting: MALE or FEMALE"
    )
    location: Optional[str] = Field(
        None, description="Location targeting for the campaign"
    )


class _BaseCampaignSchema(BaseModel):
    ad_title: str = Field(..., description="Title of the advertisement")
    ad_text: str = Field(..., description="Main text content of the advertisement")
    clicks_limit: conint(ge=0) = Field(
        ..., description="Maximum number of clicks allowed for this campaign"
    )
    cost_per_clicks: condecimal(ge=0) = Field(..., description="Cost per click (CPC)")
    cost_per_impressions: condecimal(ge=0) = Field(
        ..., description="Cost per thousand impressions (CPM)"
    )
    end_date: conint(ge=0) = Field(
        ..., description="Campaign end date (Unix timestamp)"
    )
    impressions_limit: conint(ge=0) = Field(
        ..., description="Maximum number of impressions allowed for this campaign"
    )
    start_date: conint(ge=0) = Field(
        ..., description="Campaign start date (Unix timestamp)"
    )
    targeting: TargetingCampaignSchema = Field(
        ..., description="Targeting criteria for the campaign"
    )


class CampaignsCreateRequest(_BaseCampaignSchema):
    pass


class CampaignsUpdateRequest(_BaseCampaignSchema):
    pass


class CampaignsGenerateTextRequest(BaseModel):
    ad_text: Optional[str] = Field(
        None, description="Optional existing ad text to base generation on"
    )
    ad_title: Optional[str] = Field(
        None, description="Optional existing ad title to base generation on"
    )
    generate_type: str = Field(
        ...,
        description="Type of text generation (TITLE, TEXT, or ALL)",
        pattern=r"^(TITLE|TEXT|ALL)$",
    )


class CampaignSchema(_BaseCampaignSchema):
    advertiser_id: UUID = Field(
        ..., description="Unique identifier of the advertiser who created this ad"
    )
    campaign_id: UUID = Field(..., description="Unique identifier for the campaign")

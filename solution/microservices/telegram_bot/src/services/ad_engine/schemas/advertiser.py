from pydantic import BaseModel


class AdvertiserSchema(BaseModel):
    advertiser_id: str
    name: str

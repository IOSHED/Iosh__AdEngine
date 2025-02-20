import uuid

from pydantic import BaseModel


class AdvertiserSchema(BaseModel):
    client_id: uuid.UUID
    name: str

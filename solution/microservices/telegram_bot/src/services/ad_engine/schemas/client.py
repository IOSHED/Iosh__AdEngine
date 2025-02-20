import uuid

from pydantic import BaseModel, Field, constr


class ClientProfileSchema(BaseModel):
    client_id: uuid.UUID
    login: str
    location: str
    gender: constr(pattern=r"^(MALE|FEMALE)$")
    age: int = Field(..., ge=1, le=160)

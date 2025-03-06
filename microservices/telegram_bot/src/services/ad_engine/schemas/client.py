from pydantic import BaseModel, Field, constr


class ClientProfileSchema(BaseModel):
    client_id: str
    login: str
    location: str
    gender: constr(pattern=r"^(MALE|FEMALE)$")
    age: int = Field(..., ge=1, le=160)

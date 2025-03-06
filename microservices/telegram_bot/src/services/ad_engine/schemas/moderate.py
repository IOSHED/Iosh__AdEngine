from pydantic import BaseModel


class ModerateSchema(BaseModel):
    is_activate: bool

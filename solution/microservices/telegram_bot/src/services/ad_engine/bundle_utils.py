import hashlib
import uuid


def generate_uuid_from_id(simple_id: int) -> uuid.UUID:
    hash_object = hashlib.sha256(str(simple_id).encode())
    uuid_from_id = uuid.UUID(hash_object.hexdigest()[:32])
    return uuid_from_id

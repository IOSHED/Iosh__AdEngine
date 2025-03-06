from src.config import SETTINGS


class RedisClient:
    """
    Client for interacting with Redis database.

    Attributes:
        host (str): Redis server hostname parsed from settings
        port (str): Redis server port number parsed from settings
        db (str): Redis database number from settings
    """

    host, port = SETTINGS.redis.address.split(":")
    db = SETTINGS.redis.db

    @classmethod
    def get_url(cls) -> str:
        """
        Constructs the Redis connection URL string.

        Returns:
            str: Fully formatted Redis URL in the format: redis://host:port/db

        Example:
            >>> RedisClient.get_url()
            'redis://localhost:6379/0'
        """
        return f"redis://{cls.host}:{cls.port}/{cls.db}"

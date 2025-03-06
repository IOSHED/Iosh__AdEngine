import asyncio
import os
from enum import Enum
from pathlib import Path

import yaml
from dotenv import load_dotenv
from pydantic import BaseModel
from pydantic_settings import BaseSettings


class AppEnviron(str, Enum):
    """Application environment enum.

    Defines valid environment values for the application configuration.
    """

    local = "local"
    prod = "prod"


class BotConfig(BaseModel):
    """Bot configuration model.

    Attributes:
        token (str): Bot authentication token
    """

    token: str


class RedisConfig(BaseModel):
    """Redis connection configuration.

    Attributes:
        address (str): Redis server address
        db (int): Redis database number
    """

    address: str
    db: int


class LoggerConfig(BaseModel):
    """Logging configuration model.

    Attributes:
        max_level_cmd (str): Maximum log level for command output
        max_level_file (str): Maximum log level for file output
        max_level_error_file (str): Maximum log level for error file
        log_dir (str): Directory for log files. Defaults to "./log"
    """

    max_level_cmd: str
    max_level_file: str
    max_level_error_file: str
    log_dir: str = "./log"


class AdEngineConfig(BaseModel):
    """Ad Engine configuration model.

    Attributes:
        base_url (str): URL of the Ad Engine service
    """

    base_url: str


class Config(BaseSettings):
    """Main application configuration.

    Combines all config models and handles environment variable loading.

    Attributes:
        redis (RedisConfig): Redis connection settings
        logger (LoggerConfig): Logging configuration
        bot (BotConfig): Bot-specific settings
    """

    ad_engine: AdEngineConfig
    redis: RedisConfig
    logger: LoggerConfig
    bot: BotConfig

    class Config:
        env_prefix = "APP__"
        env_file = ".env"
        env_file_encoding = "utf-8"
        env_nested_delimiter = "__"


async def load_yaml(file_path: Path) -> dict:
    """Load and parse a YAML configuration file.

    Args:
        file_path (Path): Path to the YAML file

    Returns:
        dict: Parsed YAML content

    Raises:
        yaml.YAMLError: If the YAML file is invalid
    """
    with file_path.open("r") as file:
        return yaml.safe_load(file)


async def parse_config(base_path: Path) -> Config:
    """Parse application configuration from YAML files and environment.

    Loads configuration from base.yaml and environment-specific yaml file,
    with environment variables taking precedence.

    Args:
        base_path (Path): Base path containing config files

    Returns:
        Config: Parsed configuration object

    Raises:
        ValueError: If APP_ENVIRONMENT is invalid
    """
    load_dotenv()

    environment = os.getenv("APP_ENVIRONMENT", "prod")
    if environment not in AppEnviron:
        raise ValueError("APP_ENVIRONMENT must be 'local' or 'prod'.")

    base_config = await load_yaml(base_path / "base.yaml") or {}
    env_config = await load_yaml(base_path / f"{environment}.yaml") or {}

    config_data = {**base_config, **env_config}

    return Config(**config_data)


SETTINGS = asyncio.run(parse_config(Path(__file__).parent.parent / "conf"))

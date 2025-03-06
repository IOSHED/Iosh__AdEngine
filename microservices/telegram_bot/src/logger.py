import logging
import os
from datetime import datetime
from typing import Tuple

from src.config import SETTINGS


async def get_log_filenames(log_dir: str) -> Tuple[str, str]:
    """
    Generate timestamped filenames for error and general log files.

    Args:
        log_dir (str): Directory path where log files will be created

    Returns:
        Tuple[str, str]: A tuple containing paths for error log and general log files
        in the format (error_log_path, all_log_path)
    """
    now = datetime.now()
    timestamp = now.strftime("%Y-%m-%d-%H-%M-%S")
    return f"{log_dir}/{timestamp}-error.log", f"{log_dir}/{timestamp}-all.log"


async def setup_logger() -> None:
    """
    Configure and initialize the application's logging system.

    Sets up three logging handlers:
    - File handler for all log levels
    - File handler for error logs
    - Console handler for command line output

    The log levels and directory are configured via SETTINGS.logger.
    Creates the log directory if it doesn't exist.

    Raises:
        OSError: If there are permission issues creating the log directory
        ValueError: If invalid log levels are specified in settings
    """
    log_dir = SETTINGS.logger.log_dir
    os.makedirs(log_dir, exist_ok=True)

    error_log_file, all_log_file = await get_log_filenames(log_dir)

    name_to_level = {
        "debug": logging.DEBUG,
        "info": logging.INFO,
        "warn": logging.WARNING,
        "error": logging.ERROR,
        "critical": logging.CRITICAL,
    }

    all_handler = logging.FileHandler(all_log_file)
    all_handler.setLevel(name_to_level[SETTINGS.logger.max_level_file])

    error_handler = logging.FileHandler(error_log_file)
    error_handler.setLevel(name_to_level[SETTINGS.logger.max_level_error_file])

    console_handler = logging.StreamHandler()
    console_handler.setLevel(name_to_level[SETTINGS.logger.max_level_cmd])

    logging.basicConfig(
        level=name_to_level[SETTINGS.logger.max_level_file],
        format="%(asctime)s - %(levelname)s - %(message)s",
        handlers=[
            all_handler,
            error_handler,
            console_handler,
        ],
    )

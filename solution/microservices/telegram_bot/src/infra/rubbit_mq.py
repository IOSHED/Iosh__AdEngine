import json
import logging
import uuid
from typing import Any, Dict, Optional

import aio_pika
import requests
from requests.auth import HTTPBasicAuth

from src.config import SETTINGS


class RabbitMQInterface:
    """Asynchronous RabbitMQ client implementing request-response pattern.

    This class provides an interface for asynchronous message publishing and consumption
    using RabbitMQ message broker. It implements the request-response pattern using
    correlation IDs to match responses with their corresponding requests.

    The client handles automatic connection management, queue declarations, message
    publishing with correlation IDs, and correlated response consumption.

    Attributes:
        host (str): RabbitMQ broker hostname from configuration
        port (int): RabbitMQ broker port from configuration
        username (str): Authentication username for RabbitMQ broker
        password (str): Authentication password for RabbitMQ broker
        connection (aio_pika.RobustConnection): Shared persistent connection to RabbitMQ
        channel (aio_pika.Channel): Channel for publishing and consuming messages
        request_queue (str): Name of queue used for publishing requests
        response_queue (str): Name of queue used for consuming responses
        virtualhost (str): RabbitMQ virtual host name
    """

    host = SETTINGS.rabbit_mq.host
    port = SETTINGS.rabbit_mq.port
    port_api = SETTINGS.rabbit_mq.port_api
    username = SETTINGS.rabbit_mq.username
    password = SETTINGS.rabbit_mq.password

    connection = None

    def __init__(
        self,
        request_queue: str,
        response_queue: str,
        virtualhost: str,
    ) -> None:
        """Initialize RabbitMQ interface with queue and connection settings.

        Args:
            request_queue: Name of queue to publish request messages to
            response_queue: Name of queue to consume response messages from
            virtualhost: RabbitMQ virtual host to connect to
        """
        self.request_queue = request_queue
        self.response_queue = response_queue
        self.virtualhost = virtualhost
        self.create_vhost(self.virtualhost)
        self.channel = None

    async def connect(self) -> None:
        """Establish connection to RabbitMQ and initialize channel.

        Creates a new robust connection if one doesn't exist or is closed.
        Declares durable request and response queues on the channel.

        The connection is shared across instances while channels are instance-specific.
        """
        if not self.connection or self.connection.is_closed:
            self.connection = await aio_pika.connect_robust(
                host=self.host,
                port=self.port,
                login=self.username,
                password=self.password,
                virtualhost=self.virtualhost,
            )

            self.channel = await self.connection.channel()
            await self.channel.declare_queue(self.request_queue, durable=True)
            await self.channel.declare_queue(self.response_queue, durable=True)

    async def close(self) -> None:
        """Close the RabbitMQ channel.

        Closes the channel if it exists and sets it to None.
        The connection remains open for reuse by other instances.
        """
        if self.channel:
            await self.channel.close()
            self.channel = None

    async def send_message(
        self, message_body: Dict[str, Any]
    ) -> Optional[Dict[str, Any]]:
        """Send a message and wait for the correlated response.

        Publishes a message with a unique correlation ID and waits for
        a response message with the matching correlation ID.

        Args:
            message_body: Dictionary containing the message payload to publish

        Returns:
            The correlated response message payload if received, None otherwise

        Raises:
            Exception: If message publishing or consumption fails
        """

        await self.connect()

        correlation_id = str(uuid.uuid4())

        try:
            await self.publish(message_body, correlation_id)
            response_data = await self.consume(correlation_id)
            return response_data
        except Exception as e:
            logging.error(f"Failed to send message in RabbitMq: {e}")
            await self.close()
            raise

    async def publish(self, message_body: Dict[str, Any], correlation_id: str) -> None:
        """Publish a message to the request queue.

        Publishes the message with the given correlation ID and response queue
        for routing the response back.

        Args:
            message_body: Dictionary containing the message payload
            correlation_id: Unique ID to correlate this request with its response
        """
        await self.channel.default_exchange.publish(
            aio_pika.Message(
                body=json.dumps(message_body).encode(),
                correlation_id=correlation_id,
                reply_to=self.response_queue,
            ),
            routing_key=self.request_queue,
        )

    async def consume(self, correlation_id: str) -> Dict[str, Any]:
        """Consume and return the correlated response message.

        Waits for and consumes a message from the response queue that matches
        the given correlation ID.

        Args:
            correlation_id: Correlation ID to match with the response message

        Returns:
            The response message payload if a match is found, None otherwise
        """
        queue = await self.channel.get_queue(self.response_queue)
        async with queue.iterator() as queue_iter:
            async for message in queue_iter:
                if message.correlation_id == correlation_id:
                    response_data = json.loads(message.body)
                    await message.ack()
                    return response_data
        return None

    def create_vhost(cls, vhost_name: str) -> str:
        url = f"http://{cls.host}:{cls.port_api}/api/vhosts/{vhost_name}"
        response = requests.put(url, auth=HTTPBasicAuth(cls.username, cls.password))

        match response.status_code:
            case 201:
                logging.info(f'vhost "{vhost_name}" for Rabbit Mq success created.')

            case 204:
                logging.info(f'vhost "{vhost_name}" for Rabbit Mq already exists.')

            case _:
                logging.warning(f'vhost "{vhost_name}" for Rabbit Mq.')
                raise Exception(response.text)

        return response.text

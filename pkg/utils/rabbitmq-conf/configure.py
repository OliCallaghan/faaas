import pika
import ssl
import json

class BasicPikaClient:
    def __init__(self, rabbitmq_broker_id, rabbitmq_user, rabbitmq_password, region):

        # SSL Context for TLS configuration of Amazon MQ for RabbitMQ
        ssl_context = ssl.SSLContext(ssl.PROTOCOL_TLSv1_2)
        ssl_context.set_ciphers('ECDHE+AESGCM:!ECDSA')

        url = f"amqps://{rabbitmq_user}:{rabbitmq_password}@{rabbitmq_broker_id}.mq.{region}.amazonaws.com:5671"
        parameters = pika.URLParameters(url)
        parameters.ssl_options = pika.SSLOptions(context=ssl_context)

        self.connection = pika.BlockingConnection(parameters)
        self.channel = self.connection.channel()

    def close(self):
        self.channel.close()
        self.connection.close()

class BasicPikaLambdaSetupClient(BasicPikaClient):
    def declare_queue_for_lambda(self, lambda_name: str):
        self.channel.queue_declare(queue=f"{lambda_name}-queue", durable=False)
        self.channel.queue_bind(exchange="amq.direct", queue=f"{lambda_name}-queue", routing_key=f"mq.invocations.tasks.{lambda_name}")

client = BasicPikaLambdaSetupClient(
    'b-5a738d61-9443-4566-84e9-2c576774f230',
    'admin',
    'ishouldmakethissecure',
    "eu-west-2",
)

programs = ["warehouse-report", "warehouse-order", "warehouse-pred", "pets", "bank", "echoer"]
memory_confs = [128, 256, 512, 1024]
strategies = ["adaptive", "local", "proxy"]

for program in programs:
    for memory_conf in memory_confs:
        for strategy in strategies:
            lambda_name = f"{program}-{memory_conf}-{strategy}"
            print(f"{lambda_name}-queue", "->", f"mq.invocations.tasks.{lambda_name}")
            client.declare_queue_for_lambda(lambda_name)

client.close()

from diagrams import Cluster, Diagram

from diagrams.aws.compute import Lambda, ECS, ElasticContainerServiceService
from diagrams.aws.database import RDS
from diagrams.aws.integration import MQ
from diagrams.aws.network import InternetGateway, APIGateway

graph_attr = { "pad": "0" }
cluster_style = { "fontsize": "28" }

with Diagram(show=False, filename="assets/aws-faaas-arch", graph_attr=graph_attr, outformat="svg"):
    net = InternetGateway("Internet")

    mq = MQ("RabbitMQ")
    db = RDS("Database")

    fn = Lambda("Handler")

    consumers = ECS("Proxy Service Cluster")
    gtway_cluster = ECS("Gateway Cluster")

    ingrs = APIGateway("HTTP API Ingress")
    gtway = ElasticContainerServiceService("Gateway Service")
    pgsql = ElasticContainerServiceService("SQL Proxy Service")
    fetch = ElasticContainerServiceService("HTTP Proxy Service")

    mq >> fn
    fn >> mq

    ingrs >> gtway
    gtway_cluster - gtway

    consumers - pgsql
    consumers - fetch

    pgsql >> db >> pgsql
    fetch >> net >> fetch

    mq >> pgsql >> mq
    mq >> fetch >> mq
    mq >> gtway >> mq

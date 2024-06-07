from diagrams import Cluster, Diagram

from diagrams.onprem.queue import RabbitMQ
from diagrams.onprem.database import PostgreSQL
from diagrams.onprem.network import Internet
from diagrams.oci.compute import Container, Functions, OKE
from diagrams.oci.network import ServiceGateway

cluster_style = { "fontsize": "28" }

with Diagram(show=False, filename="assets/faaas-arch", outformat="svg"):
    net = Internet("Internet")

    mq = RabbitMQ("RabbitMQ")
    db = PostgreSQL("Database")

    fn = Functions("Handler")

    consumers = OKE("Proxy Service Cluster")
    gtway_cluster = OKE("Gateway Cluster")

    ingrs = ServiceGateway("HTTP API Ingress")
    gtway = Container("Gateway Service")
    pgsql = Container("SQL Proxy Service")
    fetch = Container("HTTP Proxy Service")

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

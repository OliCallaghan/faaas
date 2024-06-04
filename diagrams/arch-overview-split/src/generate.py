from dot2tex import dot2tex
from graphviz import Source
from pydot import Dot, Node, Edge, Cluster

graph = Dot("split-overview", graph_type="digraph")

invoker = Node("invoker", label="invoker")

# Function nodes
handlers = Cluster("handlers", label="handler")
handler_0 = Node("handler_0", label="split 0", shape="box")
handler_1 = Node("handler_1", label="split 1", shape="box")

handlers.add_node(handler_0)
handlers.add_node(handler_1)
graph.add_subgraph(handlers)

# Message buses for continuations
tsk_bus_0 = Node("task_bus_0", label="topic 0", shape="circle")
tsk_bus_1 = Node("task_bus_1", label="topic 1", shape="circle")
sql_bus = Node("sql_bus", label="sql bus", shape="circle")

tsk_buses = Cluster("task_buses")
tsk_buses.add_node(tsk_bus_0)
tsk_buses.add_node(tsk_bus_1)
tsk_buses.add_node(sql_bus)

req_bus = Node("req_bus", label="req bus", shape="circle")
res_bus = Node("res_bus", label="res bus", shape="circle")

graph.add_subgraph(tsk_buses)
graph.add_node(req_bus)
graph.add_node(res_bus)

# DB and Ingress
db = Node("db", label="db", shape="cylinder")

gress = Cluster("gress", label="Ingress/Egress")
ingress = Node("ingress", label="ingress", shape="circle")
egress = Node("egress", label="egress", shape="circle")

graph.add_node(db)

# Ingrest creates task on task bus 0
ingress_to_tsk_1 = Edge("req_bus", "task_bus_0", label="invokes")

# Task bus 0 invokes handler 0
tsk_0_to_handler_0 = Edge("task_bus_0", "handler_0", label="invokes")

# Query added from handler_0 to SQL bus
handler_0_to_sql = Edge("handler_0", "sql_bus", label="queries")

# SQL bus invokes DB instance
sql_to_db = Edge("sql_bus", "db", label="invokes")

# DB instance sends response to handler_1 task bus
db_to_tsk_1 = Edge("db", "task_bus_1")

# handler_1 task bus invokes handler_1
tsk_1_to_handler_1 = Edge("task_bus_1", "handler_1", label="invokes")

# handler_1 sends response to res_bus
handler_1_to_res = Edge("handler_1", "res_bus", label="responds")


graph.add_edge(ingress_to_tsk_1)
graph.add_edge(tsk_0_to_handler_0)
graph.add_edge(handler_0_to_sql)
graph.add_edge(sql_to_db)
graph.add_edge(db_to_tsk_1)
graph.add_edge(tsk_1_to_handler_1)
graph.add_edge(handler_1_to_res)

dot = graph.to_string()

template = "<<drawcommands>>"
dot = dot2tex(dot, format="pgf", preproc=True)
tex = dot2tex(dot, format="pgf", outputfile="assets/arch-overview-split.pgf", template=template)

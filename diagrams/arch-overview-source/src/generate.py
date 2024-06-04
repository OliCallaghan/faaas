from dot2tex import dot2tex
from graphviz import Source
from pydot import Dot, Node, Edge, Cluster

graph = Dot("split-overview", graph_type="digraph")

invoker = Node("invoker", label="invoker")

# Function nodes
# Splitting Region
handler_0 = Node("handler_0", label="handler", shape="box")

graph.add_node(handler_0)

# Message buses for continuations
tsk_bus_0 = Node("task_bus", label="req bus", shape="circle")
res_bus = Node("res_bus", label="res bus", shape="circle")

graph.add_node(tsk_bus_0)
graph.add_node(res_bus)

# DB and Ingress
db = Node("db", label="db", shape="cylinder")

gress = Cluster("gress")
ingress = Node("ingress", label="ingress", shape="circle")
egress = Node("egress", label="egress", shape="circle")

graph.add_node(db)
gress.add_node(ingress)
gress.add_node(egress)
graph.add_subgraph(gress)

# Ingrest creates task on task bus 0
ingress_to_tsk_1 = Edge("ingress", "task_bus", label="creates task")

# Task bus 0 invokes handler 0
tsk_0_to_handler_0 = Edge("task_bus", "handler_0", label="invokes")

# Query added from handler_0 to SQL bus
handler_0_to_sql = Edge("handler_0", "db", label="queries")

# SQL bus invokes DB instance
sql_to_db = Edge("db", "handler_0", label="responds")

# handler_1 sends response to res_bus
handler_1_to_res = Edge("handler_0", "res_bus", label="responds")

# res bus sends response to egress
res_to_ingress = Edge("res_bus", "egress", label="responds")

invoke_req = Edge("invoker", "ingress", label="invokes")
invoke_res = Edge("egress", "invoker", label="responds")

graph.add_edge(ingress_to_tsk_1)
graph.add_edge(tsk_0_to_handler_0)
graph.add_edge(handler_0_to_sql)
graph.add_edge(sql_to_db)
graph.add_edge(handler_1_to_res)
graph.add_edge(res_to_ingress)

graph.add_edge(invoke_req)
graph.add_edge(invoke_res)

dot = graph.to_string()

template = "<<drawcommands>>"
dot = dot2tex(dot, preproc=True)
tex = dot2tex(dot, format="pgf", outputfile="assets/arch-overview-source.pgf", template=template)

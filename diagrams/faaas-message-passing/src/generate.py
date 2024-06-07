from dot2tex import dot2tex
from graphviz import Source
from pydot import Dot, Node, Edge, Cluster

## Double Billing Problem DB Request
graph = Dot("double-billing-problem", graph_type="digraph", rankdir="LR")

c0 = Cluster("c0", label="Handler 0")
c1 = Cluster("c1", label="Handler 1")

h0_cont_1 = "{<c1> handler_0 | {req = ...}}"
h0_cont_2 = "{<c2> handler_0 | {req = ...}}"
h0_cont_3 = "{<c3> handler_0 | {req = ...}}"
h0_cont_4 = "{<c4> handler_0 | {req = ...}}"

h1_cont_1 = "{<c1> handler_1 | {req = ... | pets = ...}}"
h1_cont_2 = "{<c2> handler_1 | {req = ... | pets = ...}}"
h1_cont_3 = "{<c3> handler_1 | {req = ... | pets = ...}}"
h1_cont_4 = "{<c4> handler_1 | {req = ... | pets = ...}}"

q0 = Node("q0", label=f"{h0_cont_1} | {h0_cont_2} | {h0_cont_3} | {h0_cont_4}", shape="record")
q1 = Node("q1", label=f"{h1_cont_1} | {h1_cont_2} | {h1_cont_3} | {h1_cont_4}", shape="record")

h0 = Node("h0", label="handler_0", shape="box")
h1 = Node("h1", label="handler_1", shape="box")

c0.add_node(q0)
c1.add_node(q1)

c0.add_node(h0)
c1.add_node(h1)

h0_take = Edge("q0:s", "h0:s", label="take")
h0_push = Edge("h0:n", "q1:c1", label="push")

h1_take = Edge("q1:s", "h1:s", label="take")
h1_push = Edge("h1:n", "q0:c1", label="push")

graph.add_edge(h0_take)
graph.add_edge(h0_push)

graph.add_edge(h1_take)
# graph.add_edge(h1_push)

graph.add_subgraph(c0)
graph.add_subgraph(c1)
graph.write(format="png", path="assets/faaas-message-passing.png")

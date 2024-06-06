from dot2tex import dot2tex
from graphviz import Source
from pydot import Dot, Node, Edge, Cluster

## Double Billing Problem DB Request
graph = Dot("double-billing-problem", graph_type="digraph", rankdir="LR")

fn = Node("fn", label="invoke | <db_req> query | &#8987; | &#8987; | &#8987; | &#8987; | <db_res> await | return", shape="record")
db = Node("db", label="<db_req> db | db | db | <db_res> db", shape="record")

billed_unit_of_time = Node("billed_unit_of_time", label="... | ...", shape="record")

graph.add_node(fn)
graph.add_node(db)

graph.add_node(billed_unit_of_time)

db_req = Edge("fn:db_req", "db:db_req")
db_res = Edge("db:db_res", "fn:db_res")

graph.add_edge(db_req)
graph.add_edge(db_res)

graph.write(format="pdf", path="assets/double-billed-db.pdf")

## Double Billing Problem Nested Function
graph = Dot("double-billing-problem-nest", graph_type="digraph", rankdir="LR")

fn_fst = Node("fn_fst", label="invoke | <nest_req> nest | &#8987; | &#8987; | &#8987; | &#8987; | <nest_res> await | return", shape="record")
fn_snd = Node("fn_snd", label="<nest_req> invoke | <db_req> query | &#8987; | &#8987; | <db_res> await | <nest_res> return", shape="record")
db = Node("db", label="<db_req> db | <db_res> db", shape="record")

billed_unit_of_time = Node("billed_unit_of_time", label="... | ...", shape="record")

graph.add_node(fn_fst)
graph.add_node(fn_snd)
graph.add_node(db)

graph.add_node(billed_unit_of_time)

nest_req = Edge("fn_fst:nest_req", "fn_snd:nest_req")
nest_res = Edge("fn_snd:nest_res", "fn_fst:nest_res")

db_req = Edge("fn_snd:db_req", "db:db_req")
db_res = Edge("db:db_res", "fn_snd:db_res")

graph.add_edge(db_req)
graph.add_edge(db_res)

graph.add_edge(nest_req)
graph.add_edge(nest_res)

graph.write(format="pdf", path="assets/double-billed-nested.pdf")

## Double Billing Solution
graph = Dot("double-billing-problem-split", graph_type="digraph")

fn_fst = Node("fn_fst", label="{func | <db_req> query}", shape="record")
fn_snd = Node("fn_snd", label="{<db_res> await | ret}", shape="record")
db = Node("db", label="{<db_req> db | db | db | <db_res> db}", shape="record")

graph.add_node(fn_fst)
graph.add_node(fn_snd)
graph.add_node(db)

db_req = Edge("fn_fst:db_req", "db:db_req")
db_res = Edge("db:db_res", "fn_snd:db_res")

graph.add_edge(db_req)
graph.add_edge(db_res)

state = Node("state", label="state", shape="circle")

graph.add_node(state)

fn_fst_state = Edge("fn_fst:db_req", "state")
fn_snd_state = Edge("state", "fn_snd:db_res")

graph.add_edge(fn_fst_state)
graph.add_edge(fn_snd_state)

graph.write(format="pdf", path="assets/double-billed-db-split.pdf")

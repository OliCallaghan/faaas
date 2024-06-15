from dot2tex import dot2tex
from graphviz import Source
from pydot import Dot, Node, Edge, Cluster

handler_col = "green"
pre_col = "purple"
split_col = "red"
post_col = "blue"

graph = Dot("faaasc splitting overview", graph_type="digraph")
input = Cluster("input", label="input")

mod = Node("module_in", label="module", shape="circle")
handler = Node("handler", label="handler | ctx", shape="record", color=handler_col, fontcolor=handler_col)

parent_stmt = Node("parent_stmt", label="{... | statement | statement}", shape="record", color=pre_col, fontcolor=pre_col)
use_async_directive = Node("use_async_directive", label="use async", shape="parallelogram", color=split_col, fontcolor=split_col)
target_stmt = Node("directive_target_stmt", label="declaration\nstatement | { foo | await bar(args) }", shape="record", color=split_col, fontcolor=split_col)
subseq_stmt = Node("subseq_stmt", label="{statement | statement | ...}", shape="record", color=post_col, fontcolor=post_col)

mod_to_handler = Edge("module_in", "handler", label="has item")
handler_to_parent_stmt = Edge("handler", "parent_stmt", label="has body", style="dashed", color=pre_col, fontcolor=pre_col)
parent_stmt_to_use_async_directive = Edge("parent_stmt", "use_async_directive", label="next", color=split_col, fontcolor=split_col)
use_async_directive_to_target_stmt = Edge("use_async_directive", "directive_target_stmt", label="next", color=split_col, fontcolor=split_col)
target_stmt_to_subseq_stmt = Edge("directive_target_stmt", "subseq_stmt", label="next", color=post_col, fontcolor=post_col)

input.add_node(mod)
input.add_node(handler)
input.add_node(parent_stmt)
input.add_node(use_async_directive)
input.add_node(target_stmt)
input.add_node(subseq_stmt)

input.add_edge(mod_to_handler)
input.add_edge(handler_to_parent_stmt)
input.add_edge(parent_stmt_to_use_async_directive)
input.add_edge(use_async_directive_to_target_stmt)
input.add_edge(target_stmt_to_subseq_stmt)

output = Cluster("output", label="output")

mod = Node("module_out", label="module", shape="circle")
handler = Node("handler0", label="handler_0 | ctx | state", shape="record", color=handler_col, fontcolor=handler_col)

parent_stmt = Node("parent_stmt0", label="{... | statement | statement}", shape="record", color=pre_col, fontcolor=pre_col)
target_stmt = Node("cont_stmt0", label="return\ncontinuation | { bar | args | state }", shape="record", color=split_col, fontcolor=split_col)

mod_to_handler = Edge("module_out", "handler0", label="has item")
handler_to_parent_stmt = Edge("handler0", "parent_stmt0", label="has body", style="dashed", color=pre_col, fontcolor=pre_col)
parent_stmt_to_use_async_directive = Edge("parent_stmt0", "cont_stmt0", label="next", color=split_col, fontcolor=split_col)

output.add_node(mod)
output.add_node(handler)
output.add_node(parent_stmt)
output.add_node(target_stmt)

output.add_edge(mod_to_handler)
output.add_edge(handler_to_parent_stmt)
output.add_edge(parent_stmt_to_use_async_directive)

handler = Node("handler1", label="<id> handler_1 | <decl_ctx> ctx | <decl_state> state", shape="record", color=handler_col, fontcolor=handler_col)
state_stmt = Node("state_stmt", label="declaration\nstatement | { <decl_state> ...state }", shape="record", color=split_col, fontcolor=split_col)
decl_stmt = Node("decl_stmt", label="declaration\nstatement | { foo | <decl_foo_args> ctx.args }", shape="record", color=split_col, fontcolor=split_col)
subseq_stmt = Node("subseq_stmt1", label="{statement | statement | ...}", shape="record", color=post_col, fontcolor=post_col)

mod_to_handler = Edge("module_out", "handler1", label="has item")
handler_to_state_stmt = Edge("handler1:id", "state_stmt", label="has body")
handler_state_to_state_state_stmt = Edge("handler1:decl_state", "state_stmt:decl_state", label="from")
state_stmt_to_decl_stmt = Edge("state_stmt", "decl_stmt", label="next", color=split_col, fontcolor=split_col)
handler_ctx_to_decl_foo_args = Edge("handler1:decl_ctx", "decl_stmt:decl_foo_args", label="from")
decl_stmt_to_subseq_stmt = Edge("decl_stmt", "subseq_stmt1", label="next", color=post_col, fontcolor=post_col)

output.add_node(handler)
output.add_node(state_stmt)
output.add_node(decl_stmt)
output.add_node(subseq_stmt)

output.add_edge(mod_to_handler)
output.add_edge(handler_to_state_stmt)
output.add_edge(state_stmt_to_decl_stmt)
output.add_edge(handler_state_to_state_state_stmt)
output.add_edge(handler_ctx_to_decl_foo_args)
output.add_edge(decl_stmt_to_subseq_stmt)

faaasc = Node("faaasc", label="faaasc", shape="circle")
faaasc_module_in = Edge("module_in", "faaasc", label="input")
faaasc_module_out = Edge("faaasc", "module_out", label="output")

graph.add_subgraph(input)
graph.add_subgraph(output)

graph.add_node(faaasc)
graph.add_edge(faaasc_module_in)
graph.add_edge(faaasc_module_out)

# dot = graph.to_string()
graph.write(format="svg", path="assets/faaasc-ast-split.svg")

# template = "<<drawcommands>>"
# dot = dot2tex(dot, format="pgf", preproc=True)
# tex = dot2tex(dot, format="pgf", outputfile="assets/arch-overview-split.pgf", template=template)

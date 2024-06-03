from dot2tex import dot2tex
from graphviz import Source, Digraph
from pydot import Subgraph, graph_from_dot_file, Dot
from re import sub

def generate(pprof_name: str):
    dot_file = f"assets/faas-profile-pprof-{pprof_name}.dot"
    pgf_file = f"assets/faas-profile-pprof-{pprof_name}.pgf"

    template = "<<drawcommands>>"

    graphs: list[Dot] | None = graph_from_dot_file(dot_file)

    if graphs is not None:
        graph = graphs[0]

        sgs: list[Subgraph] = graph.get_subgraph("cluster_L")
        for sg in sgs:
            for node in sg.get_node_list():
                print(node.get_name())
                sg.del_node(node.get_name())

        dot = graph.to_string()
        dot = dot.replace("\\n", " ")
        dot = sub(r"[0-9]+ms ", "", dot)
        dot = dot2tex(dot, format="pgf", preproc=True)
        tex = dot2tex(dot, format="pgf", outputfile=pgf_file, template=template)

generate("delete-pet")
generate("get-pet")
generate("get-pets")
generate("put-pet")

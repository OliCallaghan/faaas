use graphviz_rust::dot_structures::{Graph, Subgraph};

pub mod graph_block_stmt;
pub mod graph_decl;
pub mod graph_do_while_stmt;
pub mod graph_export_decl;
pub mod graph_expr;
pub mod graph_expr_stmt;
pub mod graph_fn_decl;
pub mod graph_for_in_stmt;
pub mod graph_for_of_stmt;
pub mod graph_for_stmt;
pub mod graph_function;
pub mod graph_if_stmt;
pub mod graph_module;
pub mod graph_module_decl;
pub mod graph_module_item;
pub mod graph_ret_stmt;
pub mod graph_stmt;
pub mod graph_switch_case;
pub mod graph_switch_stmt;
pub mod graph_throw_stmt;
pub mod graph_try_stmt;
pub mod graph_var_decl;
pub mod graph_while_stmt;

pub trait ToGraphvizGraph {
    fn to_graph(&self) -> Graph;
}

pub trait ToGraphvizSubgraph {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph>;
}

macro_rules! construct_id {
    ($parent:expr, $unique_id:expr, $node_type:expr) => {
        (
            format!("{}_{}_{}", $parent, $unique_id, $node_type),
            format!("sg_{}_{}_{}", $parent, $unique_id, $node_type),
        )
    };
    ($parent:expr, $node_type:expr) => {
        (
            format!("{}_{}", $parent, $node_type),
            format!("sg_{}_{}", $parent, $node_type),
        )
    };
}

pub(crate) use construct_id;

pub mod util {
    use graphviz_rust::dot_generator::*;
    use graphviz_rust::dot_structures::*;

    use super::construct_id;

    pub fn empty_node(parent: &str, unique_id: &str) -> Subgraph {
        let (node_id, sg_id) = construct_id!(parent, unique_id, "body");

        subgraph!(&sg_id;
            node!(node_id; attr!("label", unique_id)),
            edge!(node_id!(node_id) => node_id!(parent))
        )
    }
}

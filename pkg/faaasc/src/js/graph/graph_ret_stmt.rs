use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::ReturnStmt;

use super::{construct_id, util, ToGraphvizSubgraph};

impl ToGraphvizSubgraph for ReturnStmt {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        let (node_id, sg_id) = construct_id!(parent, "ret");

        let ret_arg_sg = self
            .arg
            .as_ref()
            .and_then(|arg| arg.to_subgraph(&node_id))
            .unwrap_or_else(|| util::empty_node(&node_id, "empty return"));

        Some(subgraph!(&sg_id;
            node!(&node_id; attr!("label", r#""ret stmt""#)),
            edge!(node_id!(node_id) => node_id!(parent)),
            ret_arg_sg
        ))
    }
}

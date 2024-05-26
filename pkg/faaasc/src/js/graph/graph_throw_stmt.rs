use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::ThrowStmt;

use super::{construct_id, util, ToGraphvizSubgraph};

impl ToGraphvizSubgraph for ThrowStmt {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        let (node_id, sg_id) = construct_id!(parent, "throw");

        let throw_arg_sg = self
            .arg
            .to_subgraph(&node_id)
            .unwrap_or_else(|| util::empty_node(&node_id, "empty throw"));

        Some(subgraph!(&sg_id;
            node!(&node_id; attr!("label", r#""throw stmt""#)),
            edge!(node_id!(node_id) => node_id!(parent)),
            throw_arg_sg
        ))
    }
}

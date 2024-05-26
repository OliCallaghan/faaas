use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::DoWhileStmt;

use super::{construct_id, ToGraphvizSubgraph};

impl ToGraphvizSubgraph for DoWhileStmt {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        let (node_id, sg_id) = construct_id!(parent, "dowhile");

        let test_sg = self.test.to_subgraph(&node_id).expect("test");
        let body_sg = self.body.to_subgraph(&node_id).expect("body");

        Some(subgraph!(&sg_id;
            node!(&node_id; attr!("label", r#""do while stmt""#)),
            edge!(node_id!(node_id) => node_id!(parent)),
            test_sg,
            body_sg
        ))
    }
}

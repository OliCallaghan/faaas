use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::IfStmt;

use super::{construct_id, util, ToGraphvizSubgraph};

impl ToGraphvizSubgraph for IfStmt {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        let (node_id, sg_id) = construct_id!(parent, "if");

        let (test_node_id, test_sg_id) = construct_id!(parent, "test");
        let (cons_node_id, cons_sg_id) = construct_id!(parent, "cons");
        let (alt_node_id, alt_sg_id) = construct_id!(parent, "alt");

        let test_node = subgraph!(test_sg_id;
            node!(test_node_id; attr!("label", "test")),
            edge!(node_id!(test_node_id) => node_id!(node_id))
        );
        let cons_node = subgraph!(cons_sg_id;
            node!(cons_node_id; attr!("label", "cons")),
            edge!(node_id!(cons_node_id) => node_id!(node_id))
        );
        let alt_node = subgraph!(alt_sg_id;
            node!(alt_node_id; attr!("label", "alt")),
            edge!(node_id!(alt_node_id) => node_id!(node_id))
        );

        let test_sg = self
            .test
            .to_subgraph(&alt_node_id)
            .unwrap_or_else(|| util::empty_node(&alt_node_id, "empty_test"));

        let cons_sg = self
            .cons
            .to_subgraph(&cons_node_id)
            .unwrap_or_else(|| util::empty_node(&cons_node_id, "empty_cons"));

        let alt_sg = self
            .alt
            .as_ref()
            .and_then(|alt| alt.to_subgraph(&alt_node_id))
            .unwrap_or_else(|| util::empty_node(&alt_node_id, "empty_alt"));

        Some(subgraph!(&sg_id;
            node!(&node_id; attr!("label", r#""if stmt""#)),
            edge!(node_id!(node_id) => node_id!(parent)),
            cons_sg,
            test_sg,
            alt_sg,
            cons_node,
            test_node,
            alt_node
        ))
    }
}

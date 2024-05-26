use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::ExprStmt;

use super::{construct_id, ToGraphvizSubgraph};
use crate::js::directive::Directive;

impl ToGraphvizSubgraph for ExprStmt {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        if self.is_directive("use async") {
            let (node_id, sg_id) = construct_id!(parent, "expr_stmt");

            Some(subgraph!(&sg_id;
                node!(&node_id; attr!("label", r#""use async""#), attr!("color", "red"), attr!("fontcolor", "red")),
                edge!(node_id!(&node_id) => node_id!(parent))
            ))
        } else {
            None
        }
    }
}

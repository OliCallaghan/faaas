use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::ForHead;
use swc_ecma_ast::ForInStmt;

use super::{construct_id, util, ToGraphvizSubgraph};

impl ToGraphvizSubgraph for ForInStmt {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        let (node_id, sg_id) = construct_id!(parent, "forin");

        let (left_node_id, left_sg_id) = construct_id!(parent, "left");
        let (right_node_id, right_sg_id) = construct_id!(parent, "right");
        let (body_node_id, body_sg_id) = construct_id!(parent, "body");

        let left_node = subgraph!(left_sg_id;
            node!(left_node_id; attr!("label", "left")),
            edge!(node_id!(left_node_id) => node_id!(node_id))
        );
        let right_node = subgraph!(right_sg_id;
            node!(right_node_id; attr!("label", "right")),
            edge!(node_id!(right_node_id) => node_id!(node_id))
        );
        let body_node = subgraph!(body_sg_id;
            node!(body_node_id; attr!("label", "body")),
            edge!(node_id!(body_node_id) => node_id!(node_id))
        );

        let left_sg = match &self.left {
            ForHead::UsingDecl(_) => Some(util::empty_node(&left_node_id, "using")),
            ForHead::VarDecl(var_decl) => var_decl.to_subgraph(&left_node_id),
            ForHead::Pat(_) => Some(util::empty_node(&left_node_id, "pat")),
        }
        .unwrap_or_else(|| util::empty_node(&left_node_id, "empty_left"));

        let right_sg = self
            .right
            .to_subgraph(&right_node_id)
            .unwrap_or_else(|| util::empty_node(&right_node_id, "empty_right"));

        let body_sg = self
            .body
            .to_subgraph(&body_node_id)
            .unwrap_or_else(|| util::empty_node(&body_node_id, "empty_body"));

        Some(subgraph!(&sg_id;
            node!(&node_id; attr!("label", r#""for in stmt""#)),
            edge!(node_id!(node_id) => node_id!(parent)),
            right_sg,
            left_sg,
            body_sg,
            right_node,
            left_node,
            body_node
        ))
    }
}

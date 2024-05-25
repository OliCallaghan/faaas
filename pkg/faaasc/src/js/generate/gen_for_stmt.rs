use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::ForStmt;

use super::{construct_id, util, ToGraphvizSubgraph};

impl ToGraphvizSubgraph for ForStmt {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        let (node_id, sg_id) = construct_id!(parent, "for");

        let (init_node_id, init_sg_id) = construct_id!(parent, "init");
        let (test_node_id, test_sg_id) = construct_id!(parent, "test");
        let (update_node_id, update_sg_id) = construct_id!(parent, "update");
        let (body_node_id, body_sg_id) = construct_id!(parent, "body");

        let init_node = subgraph!(init_sg_id;
            node!(init_node_id; attr!("label", "init")),
            edge!(node_id!(init_node_id) => node_id!(node_id))
        );
        let test_node = subgraph!(test_sg_id;
            node!(test_node_id; attr!("label", "test")),
            edge!(node_id!(test_node_id) => node_id!(node_id))
        );
        let update_node = subgraph!(update_sg_id;
            node!(update_node_id; attr!("label", "update")),
            edge!(node_id!(update_node_id) => node_id!(node_id))
        );
        let body_node = subgraph!(body_sg_id;
            node!(body_node_id; attr!("label", "body")),
            edge!(node_id!(body_node_id) => node_id!(node_id))
        );

        let init_sg = self
            .init
            .as_ref()
            .and_then(|init| match init {
                swc_ecma_ast::VarDeclOrExpr::Expr(expr) => expr.to_subgraph(&init_node_id),
                swc_ecma_ast::VarDeclOrExpr::VarDecl(var_decl) => {
                    var_decl.to_subgraph(&init_node_id)
                }
            })
            .unwrap_or_else(|| util::empty_node(&init_node_id, "empty_init"));

        let test_sg = self
            .test
            .as_ref()
            .and_then(|test| test.to_subgraph(&test_node_id))
            .unwrap_or_else(|| util::empty_node(&test_node_id, "empty_test"));

        let update_sg = self
            .update
            .as_ref()
            .and_then(|update| update.to_subgraph(&update_node_id))
            .unwrap_or_else(|| util::empty_node(&update_node_id, "empty_update"));

        let body_sg = self
            .body
            .to_subgraph(&body_node_id)
            .unwrap_or_else(|| util::empty_node(&body_node_id, "empty_body"));

        Some(subgraph!(&sg_id;
            node!(&node_id; attr!("label", r#""for stmt""#)),
            edge!(node_id!(node_id) => node_id!(parent)),
            init_sg,
            test_sg,
            update_sg,
            body_sg,
            init_node,
            test_node,
            update_node,
            body_node
        ))
    }
}

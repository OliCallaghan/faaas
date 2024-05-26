use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::FnDecl;

use super::{construct_id, ToGraphvizSubgraph};

impl ToGraphvizSubgraph for FnDecl {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        let fn_name = &self.ident.sym.as_str();
        let label = format!(r#""fn {}""#, fn_name);

        let (node_id, sg_id) = construct_id!(parent, fn_name, "fn_name");

        let fn_sg = self.function.to_subgraph(&node_id).unwrap_or_else(|| {
            let (body_node_id, body_sg_id) = construct_id!(node_id, "body");

            subgraph!(&body_sg_id;
                node!(body_node_id; attr!("label", "empty")),
                edge!(node_id!(body_node_id) => node_id!(node_id))
            )
        });

        Some(subgraph!(&sg_id;
            node!(&node_id; attr!("label", &label)),
            edge!(node_id!(&node_id) => node_id!(parent)),
            fn_sg
        ))
    }
}

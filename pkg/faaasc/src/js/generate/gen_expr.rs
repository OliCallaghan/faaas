use anyhow::Result;
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::Expr;

use super::{construct_id, GenerateHandler, Generation, ToGraphvizSubgraph};

impl GenerateHandler for Expr {
    fn generate_split(&self, gen: &mut Generation) -> Result<()> {
        match self {
            // Expr::Fn(fn_decl) => fn_decl.split(),
            _ => Ok(()),
        }
    }
}

impl ToGraphvizSubgraph for Expr {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        let (node_id, sg_id) = construct_id!(parent, "expr");

        Some(subgraph!(&sg_id;
                node!(node_id; attr!("label", "expr")),
                edge!(node_id!(node_id) => node_id!(parent))
        ))
    }
}

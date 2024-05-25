use anyhow::Result;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::Decl;

use super::{GenerateHandlers, ToGraphvizSubgraph};

impl GenerateHandlers for Decl {
    fn generate_handlers(&self, gen_target: &mut super::GenerationTarget) -> Result<()> {
        match self {
            Decl::Fn(fn_decl) => fn_decl.generate_handlers(gen_target),
            _ => Ok(()),
        }
    }
}

impl ToGraphvizSubgraph for Decl {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        match self {
            Decl::Fn(fn_decl) => fn_decl.to_subgraph(parent),
            Decl::Var(var_decl) => var_decl.to_subgraph(parent),
            _ => None,
        }
    }
}

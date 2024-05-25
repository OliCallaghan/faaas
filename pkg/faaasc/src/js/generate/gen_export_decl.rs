use anyhow::Result;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::ExportDecl;

use super::{GenerateHandlers, ToGraphvizSubgraph};

impl GenerateHandlers for ExportDecl {
    fn generate_handlers(&self, gen_target: &mut super::GenerationTarget) -> Result<()> {
        self.decl.generate_handlers(gen_target)
    }
}

impl ToGraphvizSubgraph for ExportDecl {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        self.decl.to_subgraph(parent)
    }
}

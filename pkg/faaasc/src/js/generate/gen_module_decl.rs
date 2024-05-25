use anyhow::Result;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::ModuleDecl;

use super::{GenerateHandlers, GenerationTarget, ToGraphvizSubgraph};

impl GenerateHandlers for ModuleDecl {
    fn generate_handlers(&self, gen_target: &mut GenerationTarget) -> Result<()> {
        match self {
            ModuleDecl::ExportDecl(decl) => decl.generate_handlers(gen_target),
            _ => Ok(()),
        }
    }
}

impl ToGraphvizSubgraph for ModuleDecl {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        match self {
            ModuleDecl::ExportDecl(decl) => decl.to_subgraph(parent),
            _ => None,
        }
    }
}

use anyhow::Result;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::ModuleItem;

use super::{GenerateHandlers, GenerationTarget, ToGraphvizSubgraph};

impl GenerateHandlers for ModuleItem {
    fn generate_handlers(&self, gen_target: &mut GenerationTarget) -> Result<()> {
        match self {
            ModuleItem::ModuleDecl(decl) => decl.generate_handlers(gen_target),
            _ => Ok(()),
        }
    }
}

impl ToGraphvizSubgraph for ModuleItem {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        match self {
            ModuleItem::ModuleDecl(decl) => decl.to_subgraph(parent),
            _ => None,
        }
    }
}

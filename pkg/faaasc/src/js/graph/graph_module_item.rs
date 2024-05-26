use graphviz_rust::dot_structures::*;
use swc_ecma_ast::ModuleItem;

use super::ToGraphvizSubgraph;

impl ToGraphvizSubgraph for ModuleItem {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        match self {
            ModuleItem::ModuleDecl(decl) => decl.to_subgraph(parent),
            _ => None,
        }
    }
}

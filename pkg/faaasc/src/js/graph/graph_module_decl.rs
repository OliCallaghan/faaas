use graphviz_rust::dot_structures::*;
use swc_ecma_ast::ModuleDecl;

use super::ToGraphvizSubgraph;

impl ToGraphvizSubgraph for ModuleDecl {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        match self {
            ModuleDecl::ExportDecl(decl) => decl.to_subgraph(parent),
            _ => None,
        }
    }
}

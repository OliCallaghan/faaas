use graphviz_rust::dot_structures::*;
use swc_ecma_ast::ExportDecl;

use super::ToGraphvizSubgraph;

impl ToGraphvizSubgraph for ExportDecl {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        self.decl.to_subgraph(parent)
    }
}

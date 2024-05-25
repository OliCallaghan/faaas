use anyhow::Result;
use graphviz_rust::dot_structures::Subgraph;
use swc_ecma_ast::Stmt;

use super::{GenerateHandler, Generation, ToGraphvizSubgraph};

impl GenerateHandler for Stmt {
    fn generate_split(&self, gen: &mut Generation) -> Result<()> {
        match self {
            Stmt::Block(block_stmt) => block_stmt.generate_split(gen),
            Stmt::Expr(expr_stmt) => expr_stmt.generate_split(gen),
            _ => Ok(()),
        }
    }
}

impl ToGraphvizSubgraph for Stmt {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        match self {
            Stmt::Block(block_stmt) => block_stmt.to_subgraph(parent),
            Stmt::Expr(expr_stmt) => expr_stmt.to_subgraph(parent),
            Stmt::Decl(decl_stmt) => decl_stmt.to_subgraph(parent),
            _ => None,
        }
    }
}

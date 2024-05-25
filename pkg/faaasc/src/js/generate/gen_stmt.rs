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
            Stmt::Decl(decl_stmt) => decl_stmt.to_subgraph(parent),
            Stmt::DoWhile(do_while_stmt) => do_while_stmt.to_subgraph(parent),
            Stmt::Expr(expr_stmt) => expr_stmt.to_subgraph(parent),
            Stmt::For(for_stmt) => for_stmt.to_subgraph(parent),
            Stmt::ForIn(for_in_stmt) => for_in_stmt.to_subgraph(parent),
            Stmt::ForOf(for_of_stmt) => for_of_stmt.to_subgraph(parent),
            Stmt::If(if_stmt) => if_stmt.to_subgraph(parent),
            Stmt::Return(ret_stmt) => ret_stmt.to_subgraph(parent),
            Stmt::Switch(switch_stmt) => switch_stmt.to_subgraph(parent),
            Stmt::Throw(throw_stmt) => throw_stmt.to_subgraph(parent),
            Stmt::Try(try_stmt) => try_stmt.to_subgraph(parent),
            Stmt::While(while_stmt) => while_stmt.to_subgraph(parent),
            _ => None,
        }
    }
}

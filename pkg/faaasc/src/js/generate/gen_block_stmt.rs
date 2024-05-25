use anyhow::Result;
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::BlockStmt;

use super::{construct_id, GenerateHandler, Generation, ToGraphvizSubgraph};

impl GenerateHandler for BlockStmt {
    fn generate_split(&self, gen: &mut Generation) -> Result<()> {
        for stmt in &self.stmts {
            let is_async_marker = stmt.generate_split(gen)?;

            if is_async_marker != () {
                // Create a new generation and make sure its been added to target.
                gen.push_stmt(stmt);
            }
        }

        Ok(())
    }
}

impl ToGraphvizSubgraph for BlockStmt {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        let (node_id, sg_id) = construct_id!(parent, "block_stmt");

        let stmts = self
            .stmts
            .iter()
            .scan(node_id.clone(), |parent, stmt| {
                let (node_id, _) = construct_id!(parent, "stmt");
                let node = node!(&node_id; attr!("label", "stmt"));
                let edge = edge!(node_id!(node_id) => node_id!(parent); attr!("label", "next"));

                let stmt_sg = stmt.to_subgraph(&node_id);

                *parent = node_id;

                Some([
                    Some(stmt!(node)),
                    Some(stmt!(edge)),
                    stmt_sg.map(|g| stmt!(g)),
                ])
            })
            .flatten()
            .flatten();

        let stmts = stmts
            .chain([
                stmt!(node!(&node_id; attr!("label", "blk_stmt"))),
                stmt!(edge!(node_id!(&node_id) => node_id!(parent))),
            ])
            .collect::<Vec<Stmt>>();

        Some(subgraph!(&sg_id, stmts))
    }
}

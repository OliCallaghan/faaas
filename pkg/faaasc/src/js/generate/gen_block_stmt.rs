use anyhow::Result;
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::BlockStmt;

use super::{GenerateHandler, Generation, ToGraphvizSubgraph};

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
        let n_id = format!("{}_block", parent);
        let sg_id = format!("sg_{}", n_id);

        let stmts = self
            .stmts
            .iter()
            .scan(n_id.clone(), |parent, _stmt| {
                let n_id = format!("{}_stmt", parent);
                let n = node!(&n_id; attr!("label", "stmt"));
                let e = edge!(node_id!(n_id) => node_id!(parent); attr!("label", "next"));

                let stmt_sg = _stmt.to_subgraph(&n_id);

                *parent = n_id;

                Some([Some(stmt!(n)), Some(stmt!(e)), stmt_sg.map(|g| stmt!(g))])
            })
            .flatten()
            .flatten();

        let stmts = stmts
            .chain([
                stmt!(node!(&n_id; attr!("label", "blk_stmt"))),
                stmt!(edge!(node_id!(&n_id) => node_id!(parent))),
            ])
            .collect::<Vec<Stmt>>();

        Some(subgraph!(&sg_id, stmts))
    }
}

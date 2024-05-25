use anyhow::Result;
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::ExprStmt;

use super::EvalLit;
use super::GenerateHandler;
use super::Generation;
use super::ToGraphvizSubgraph;

trait IsStringLiteralStmt {
    fn is_str_lit_stmt(&self, to_match: &str) -> bool;
}

impl IsStringLiteralStmt for ExprStmt {
    fn is_str_lit_stmt(&self, to_match: &str) -> bool {
        self.expr
            .eval_lit_str()
            .map_or(false, |str_val| str_val.value.as_str() == to_match)
    }
}

impl GenerateHandler for ExprStmt {
    fn generate_split(&self, gen: &mut Generation) -> Result<()> {
        if self.is_str_lit_stmt("use async") {
            println!("Found 'use async' literal inside {}", gen.name.sym.as_str());
        }

        self.expr.generate_split(gen)
    }
}

impl ToGraphvizSubgraph for ExprStmt {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        if self.is_str_lit_stmt("use async") {
            let n_id = format!("{}_expr_stmt", parent);
            let sg_id = format!("sg_{}", n_id);

            Some(subgraph!(&sg_id;
                node!(&n_id; attr!("label", r#""use async""#), attr!("color", "red"), attr!("fontcolor", "red")),
                edge!(node_id!(&n_id) => node_id!(parent))
            ))
        } else {
            None
        }
    }
}

// graphviz_rust::attributes::fillcolor;

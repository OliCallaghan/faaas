use anyhow::Result;
use graphviz_rust::dot_structures::{Graph, Subgraph};
use swc_ecma_ast::{Ident, Stmt};

pub mod gen_block_stmt;
pub mod gen_decl;
pub mod gen_do_while_stmt;
pub mod gen_export_decl;
pub mod gen_expr;
pub mod gen_expr_stmt;
pub mod gen_fn_decl;
pub mod gen_for_in_stmt;
pub mod gen_for_of_stmt;
pub mod gen_for_stmt;
pub mod gen_function;
pub mod gen_if_stmt;
pub mod gen_module;
pub mod gen_module_decl;
pub mod gen_module_item;
pub mod gen_ret_stmt;
pub mod gen_stmt;
pub mod gen_switch_case;
pub mod gen_switch_stmt;
pub mod gen_throw_stmt;
pub mod gen_try_stmt;
pub mod gen_var_decl;
pub mod gen_while_stmt;

use super::eval_lit::EvalLit;

#[derive(Default)]
pub struct GenerationTarget {
    generations: Vec<Generation>,
}

impl GenerationTarget {
    pub fn new_gen(&mut self, id: Ident) -> &mut Generation {
        self.generations.push(Generation::new(id));

        let index = self.generations.len() - 1;

        self.generations.get_mut(index).unwrap()
    }
}

pub struct Generation {
    pub name: Ident,
    pub part: u32,
    pub stmts: Vec<Stmt>,
}

impl Generation {
    pub fn new(name: Ident) -> Self {
        Self {
            name,
            part: 0,
            stmts: Default::default(),
        }
    }

    pub fn split(&self) -> Self {
        Self {
            name: self.name.clone(),
            part: self.part + 1,
            stmts: Default::default(),
        }
    }

    pub fn push_stmt(&mut self, stmt: &Stmt) {
        self.stmts.push(stmt.clone())
    }

    pub fn to_export_handler(&self) -> () {}
}

pub trait GenerateHandler {
    fn generate_split(&self, gen: &mut Generation) -> Result<()>;
}

pub trait GenerateHandlers {
    fn generate_handlers(&self, gen_target: &mut GenerationTarget) -> Result<()>;
}

pub trait Generate {
    fn generate(&mut self) -> Result<()>;
}

pub trait ToGraphvizGraph {
    fn to_graph(&self) -> Graph;
}

pub trait ToGraphvizSubgraph {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph>;
}

macro_rules! construct_id {
    ($parent:expr, $unique_id:expr, $node_type:expr) => {
        (
            format!("{}_{}_{}", $parent, $unique_id, $node_type),
            format!("sg_{}_{}_{}", $parent, $unique_id, $node_type),
        )
    };
    ($parent:expr, $node_type:expr) => {
        (
            format!("{}_{}", $parent, $node_type),
            format!("sg_{}_{}", $parent, $node_type),
        )
    };
}

use construct_id;

pub mod util {
    use graphviz_rust::dot_generator::*;
    use graphviz_rust::dot_structures::*;

    pub fn empty_node(parent: &str, unique_id: &str) -> Subgraph {
        let (node_id, sg_id) = construct_id!(parent, unique_id, "body");

        subgraph!(&sg_id;
            node!(node_id; attr!("label", unique_id)),
            edge!(node_id!(node_id) => node_id!(parent))
        )
    }
}

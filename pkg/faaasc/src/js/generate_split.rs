use anyhow::Result;

pub mod gen_block_stmt;
pub mod gen_decl;
pub mod gen_export_decl;
pub mod gen_expr;
pub mod gen_expr_stmt;
pub mod gen_fn_decl;
pub mod gen_function;
pub mod gen_module;
pub mod gen_module_decl;
pub mod gen_module_item;
pub mod gen_stmt;

use super::eval_lit::EvalLit;

pub trait GenerateSplit {
    fn generate_split(&self) -> Result<()>;
}

use anyhow::Result;
use swc_common::util::take::Take;
use swc_ecma_ast::{
    BlockStmt, Decl, ExportDecl, FnDecl, Function, Ident, ModuleDecl, ModuleItem, Stmt,
};

pub mod gen_block_stmt;
pub mod gen_decl;
pub mod gen_export_decl;
pub mod gen_fn_decl;
pub mod gen_function;
pub mod gen_module;
pub mod gen_module_decl;
pub mod gen_module_item;

use super::capture::FreeVariables;

pub trait Generate {
    fn generate(&mut self) -> Result<()>;
}

pub trait GenerateWithTarget {
    fn generate(&self, gen_target: &mut Vec<Generation>) -> Result<()>;
}

pub trait GenerateWithTargetAndIdent {
    fn generate(&self, gen_target: &mut Vec<Generation>, id: Ident) -> Result<()>;
}

pub struct Generation {
    handler_name: Ident,
    handler_split_id: u32,
    stmts: Vec<Stmt>,
    free_vars: Option<FreeVariables>,
}

impl Generation {
    pub fn new(handler_name: Ident) -> Self {
        Self {
            handler_name,
            handler_split_id: 0,
            stmts: vec![],
            free_vars: None,
        }
    }

    pub fn push_stmt(&mut self, stmt: Stmt) {
        self.stmts.push(stmt)
    }

    pub fn next(&self) -> Self {
        Self {
            handler_name: self.handler_name.clone(),
            handler_split_id: self.handler_split_id + 1,
            stmts: vec![],
            free_vars: None,
        }
    }

    pub fn to_module_item(&self) -> ModuleItem {
        let fn_ident_str = format!(
            "{}_{}",
            self.handler_name.sym.as_str(),
            self.handler_split_id
        );
        let fn_ident_atom = swc_atoms::Atom::new(fn_ident_str);
        let fn_ident = Ident::new(fn_ident_atom, Default::default());

        let function = Function {
            span: Default::default(),
            params: vec![],
            decorators: vec![],
            body: Some(BlockStmt {
                span: Default::default(),
                stmts: self.stmts.clone(),
            }),
            is_async: true,
            is_generator: false,
            type_params: None,
            return_type: None,
        };

        ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
            span: Default::default(),
            decl: Decl::Fn(FnDecl {
                ident: fn_ident,
                function: Box::new(function),
                declare: false,
            }),
        }))
    }
}

use std::collections::HashSet;

use swc_ecma_ast::{Id, Ident};

pub mod cap_block_stmt;
pub mod cap_break_stmt;
pub mod cap_continue_stmt;
pub mod cap_decl;
pub mod cap_do_while_stmt;
pub mod cap_expr;
pub mod cap_expr_stmt;
pub mod cap_for_in_stmt;
pub mod cap_for_of_stmt;
pub mod cap_for_stmt;
pub mod cap_if_stmt;
pub mod cap_pat;
pub mod cap_ret_stmt;
pub mod cap_stmt;
pub mod cap_switch_stmt;
pub mod cap_throw_stmt;
pub mod cap_try_stmt;
pub mod cap_var_decl;
pub mod cap_while_stmt;

#[derive(Clone, Debug)]
pub struct FreeVariables {
    decl: HashSet<Id>,
    free: HashSet<Id>,
}

impl FreeVariables {
    pub fn new() -> Self {
        Self {
            decl: Default::default(),
            free: Default::default(),
        }
    }

    pub fn push_decl(&mut self, decl_ident: &Ident) {
        self.decl.insert(decl_ident.to_id());
    }

    pub fn push_usage(&mut self, used_ident: &Ident) {
        let id = used_ident.to_id();

        // TODO: Capture globals so that they don't need to manually be escaped here.
        if ["sql", "listUserPets", "getUsername", "result", "console"]
            .contains(&used_ident.sym.as_str())
        {
            return;
        }

        // Check if variable is free?
        if !self.decl.contains(&id) {
            // Variable is free!
            self.free.insert(id);
        }
    }

    pub fn get(&self) -> Vec<Id> {
        self.free.iter().map(|id| id.clone()).collect()
    }
}

pub trait CaptureFreeVariables {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables);
}

#[cfg(test)]
mod tests {
    use super::*;

    use swc_common::sync::Lrc;
    use swc_common::{FileName, SourceMap};
    use swc_ecma_ast::Script;
    use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

    pub fn parse_script(script: &str) -> Script {
        let cm: Lrc<SourceMap> = Default::default();

        let filename = FileName::Custom("test.js".into());
        let fm = cm.new_source_file(filename, script.into());

        let lexer = Lexer::new(
            Syntax::Typescript(Default::default()),
            Default::default(),
            StringInput::from(&*fm),
            None,
        );

        let mut parser = Parser::new_from(lexer);

        parser.parse_script().expect("failed to parse script")
    }

    pub fn compute_free_vars(script: Script) -> FreeVariables {
        let mut free_vars = FreeVariables::new();

        for stmt in script.body {
            stmt.capture_free_vars(&mut free_vars);
        }

        free_vars
    }
}

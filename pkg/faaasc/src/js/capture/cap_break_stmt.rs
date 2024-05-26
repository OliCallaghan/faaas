use swc_ecma_ast::BreakStmt;

use super::{CaptureFreeVariables, FreeVariables};

impl CaptureFreeVariables for BreakStmt {
    fn capture_free_vars(&self, _free_vars: &mut FreeVariables) {
        if self.label.is_some() {
            unimplemented!("Break to label is unimplemented!")
        }
    }
}

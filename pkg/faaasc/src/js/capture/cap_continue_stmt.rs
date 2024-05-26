use swc_ecma_ast::ContinueStmt;

use super::{CaptureFreeVariables, FreeVariables};

impl CaptureFreeVariables for ContinueStmt {
    fn capture_free_vars(&self, _free_vars: &mut FreeVariables) {
        if self.label.is_some() {
            unimplemented!("Continue to label is unimplemented!")
        }
    }
}

use swc_ecma_ast::ExprStmt;

use super::{CaptureFreeVariables, FreeVariables};

impl CaptureFreeVariables for ExprStmt {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.expr.capture_free_vars(free_vars);
    }
}

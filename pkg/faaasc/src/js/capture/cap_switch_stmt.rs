use swc_ecma_ast::{SwitchCase, SwitchStmt};

use super::{CaptureFreeVariables, FreeVariables};

impl CaptureFreeVariables for SwitchStmt {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.discriminant.capture_free_vars(free_vars);

        for case in &self.cases {
            case.capture_free_vars(free_vars);
        }
    }
}

impl CaptureFreeVariables for SwitchCase {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        if let Some(test) = &self.test {
            test.capture_free_vars(free_vars);
        }

        for stmt in &self.cons {
            stmt.capture_free_vars(free_vars);
        }
    }
}

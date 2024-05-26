use swc_ecma_ast::{CatchClause, TryStmt};

use super::{CaptureFreeVariables, FreeVariables};

impl CaptureFreeVariables for TryStmt {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.block.capture_free_vars(free_vars);

        if let Some(handler) = &self.handler {
            handler.capture_free_vars(free_vars);
        }

        if let Some(finalizer) = &self.finalizer {
            finalizer.capture_free_vars(free_vars);
        }
    }
}

impl CaptureFreeVariables for CatchClause {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        if let Some(param) = &self.param {
            param.capture_free_vars(free_vars);
        }

        self.body.capture_free_vars(free_vars);
    }
}

#[cfg(test)]
mod tests {
    use crate::js::capture::tests::*;

    #[test]
    fn try_stmt_body_use_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            try {
                let foo = 0;
            }
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn try_stmt_body_use_free_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            try {
                foo = 5;
            }
            "#,
        ));

        let fv_id = swc_atoms::atom!("foo");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }

    #[test]
    fn try_stmt_catch_use_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let bar = 0;
            try {} catch (err) {
                let foo = bar;
            }
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn try_stmt_catch_use_err_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            try {} catch (err) {
                let foo = err;
            }
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn try_stmt_catch_use_free_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            try {} catch (err) {
                let bar = foo;
            }
            "#,
        ));

        let fv_id = swc_atoms::atom!("foo");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }

    #[test]
    fn try_stmt_finally_use_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            try {} catch (err) {} finally {
                let foo = 0;
            }
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn try_stmt_finally_use_free_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            try {} catch (err) {} finally {
                foo = 5;
            }
            "#,
        ));

        let fv_id = swc_atoms::atom!("foo");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }
}

use swc_ecma_ast::IfStmt;

use super::{CaptureFreeVariables, FreeVariables};

impl CaptureFreeVariables for IfStmt {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.test.capture_free_vars(free_vars);
        self.cons.capture_free_vars(free_vars);

        if let Some(alt) = &self.alt {
            alt.capture_free_vars(free_vars);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::js::capture::tests::*;

    #[test]
    fn if_stmt_cond_use_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = true;
            if (foo) {}
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn if_stmt_cond_use_free_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            if (foo) {}
            "#,
        ));

        let fv_id = swc_atoms::atom!("foo");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }

    #[test]
    fn if_stmt_cons_use_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = true;
            if (true) {
                foo = false;
            }
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn if_stmt_cons_use_free_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            if (true) {
                foo = false;
            }
            "#,
        ));

        let fv_id = swc_atoms::atom!("foo");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }

    #[test]
    fn if_stmt_alt_use_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = true;
            if (true) {} else {
                foo = false;
            }
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn if_stmt_alt_use_free_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            if (true) {} else {
                foo = false;
            }
            "#,
        ));

        let fv_id = swc_atoms::atom!("foo");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }
}

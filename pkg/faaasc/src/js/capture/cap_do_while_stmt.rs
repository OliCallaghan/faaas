use swc_ecma_ast::DoWhileStmt;

use super::{CaptureFreeVariables, FreeVariables};

impl CaptureFreeVariables for DoWhileStmt {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.body.capture_free_vars(free_vars);
        self.test.capture_free_vars(free_vars);
    }
}

#[cfg(test)]
mod tests {
    use crate::js::capture::tests::*;

    #[test]
    fn do_while_body_use_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = 0;
            do {
                foo = 5;
            } while(true)
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn do_while_body_use_free_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            do {
                foo = 5;
            } while(true);
            "#,
        ));

        let fv_id = swc_atoms::atom!("foo");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }

    #[test]
    fn do_while_cond_use_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = true;
            do {} while(foo);
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn do_while_cond_use_free_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            do {} while(foo);
            "#,
        ));

        let fv_id = swc_atoms::atom!("foo");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }
}

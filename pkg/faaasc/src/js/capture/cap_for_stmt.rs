use swc_ecma_ast::{ForStmt, VarDeclOrExpr};

use super::{CaptureFreeVariables, FreeVariables};

impl CaptureFreeVariables for ForStmt {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        if let Some(init) = &self.init {
            init.capture_free_vars(free_vars);
        }

        if let Some(test) = &self.test {
            test.capture_free_vars(free_vars);
        }

        if let Some(update) = &self.update {
            update.capture_free_vars(free_vars);
        }

        self.body.capture_free_vars(free_vars);
    }
}

impl CaptureFreeVariables for VarDeclOrExpr {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        match self {
            VarDeclOrExpr::Expr(expr) => expr.capture_free_vars(free_vars),
            VarDeclOrExpr::VarDecl(var_decl) => var_decl.capture_free_vars(free_vars),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::js::capture::tests::*;

    #[test]
    fn for_stmt_init_use_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = 0;
            for (let i = foo; i < 10; i++) {
                let bar = 0;
            }
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn for_stmt_init_use_free_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            for (let i = foo; i < 10; i++) {
                let bar = 0;
            }
            "#,
        ));

        let fv_id = swc_atoms::atom!("foo");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }

    #[test]
    fn for_stmt_cond_use_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = 10;
            for (let i = 0; i < foo; i++) {
                let bar = 0;
            }
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn for_stmt_cond_use_free_rhs_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            for (let i = 0; i < foo; i++) {
                let bar = 0;
            }
            "#,
        ));

        let fv_id = swc_atoms::atom!("foo");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }

    #[test]
    fn for_stmt_cond_use_free_lhs_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            for (let i = 0; foo < 10; i++) {
                let bar = 0;
            }
            "#,
        ));

        let fv_id = swc_atoms::atom!("foo");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }

    #[test]
    fn for_stmt_update_use_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            for (let i = 0; i < 10; i++) {
                let bar = 0;
            }
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn for_stmt_update_use_free_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            for (let i = 0; i < 10; foo++) {
                let bar = 0;
            }
            "#,
        ));

        let fv_id = swc_atoms::atom!("foo");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }

    #[test]
    fn for_stmt_body_use_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = 0;
            for (let i = 0; i < 10; i++) {
                foo = 0;
            }
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn for_stmt_body_use_free_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            for (let i = 0; i < 10; i++) {
                foo = 0;
            }
            "#,
        ));

        let fv_id = swc_atoms::atom!("foo");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }
}

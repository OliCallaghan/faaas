use swc_ecma_ast::BlockStmt;

use super::{CaptureFreeVariables, FreeVariables};

impl CaptureFreeVariables for BlockStmt {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        for stmt in &self.stmts {
            stmt.capture_free_vars(free_vars);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::js::capture::tests::*;

    #[test]
    fn block_ass_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = 0;
            {
                foo = 5;
            }
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn block_ass_bound_var_multiple() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = 0;
            let bar = 0;
            {
                foo = 5;
                bar = 5;
            }
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn block_ass_free_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            {
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
    fn block_ass_free_var_multiple() {
        let fv = compute_free_vars(parse_script(
            r#"
            {
                foo = 5;
                bar = 0;
            }
            "#,
        ));

        let fv1_id = swc_atoms::atom!("foo");
        let fv1_sc = swc_common::SyntaxContext::from_u32(0);

        let fv2_id = swc_atoms::atom!("bar");
        let fv2_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 2);
        assert!(fv.get().contains(&(fv1_id, fv1_sc)));
        assert!(fv.get().contains(&(fv2_id, fv2_sc)));
    }
}

use swc_ecma_ast::ReturnStmt;

use super::{CaptureFreeVariables, FreeVariables};

impl CaptureFreeVariables for ReturnStmt {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        if let Some(arg) = &self.arg {
            arg.capture_free_vars(free_vars);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::js::capture::tests::*;

    #[test]
    fn ret_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = true;
            return foo;
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn ret_free_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            return foo;
            "#,
        ));

        let fv_id = swc_atoms::atom!("foo");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }
}

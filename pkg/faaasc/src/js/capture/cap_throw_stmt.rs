use swc_ecma_ast::ThrowStmt;

use super::{CaptureFreeVariables, FreeVariables};

impl CaptureFreeVariables for ThrowStmt {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.arg.capture_free_vars(free_vars);
    }
}

#[cfg(test)]
mod tests {
    use crate::js::capture::tests::*;

    #[test]
    fn throw_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = true;
            throw foo;
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn throw_free_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            throw foo;
            "#,
        ));

        let fv_id = swc_atoms::atom!("foo");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }
}

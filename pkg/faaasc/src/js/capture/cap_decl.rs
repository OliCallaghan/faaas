use swc_ecma_ast::Decl;

use super::{CaptureFreeVariables, FreeVariables};

impl CaptureFreeVariables for Decl {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        match self {
            Decl::Var(var_decl) => var_decl.capture_free_vars(free_vars),
            _ => unimplemented!("Capturing scope from this type is not implemented yet!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::js::capture::tests::*;

    #[test]
    fn decl_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = 0;
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn decl_var_multiple() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = 0;
            let bar = 0;
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn decl_var_from_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = 0;
            let bar = foo;
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn decl_var_from_free_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = 0;
            let bar = buz;
            "#,
        ));

        let fv_id = swc_atoms::atom!("buz");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }
}

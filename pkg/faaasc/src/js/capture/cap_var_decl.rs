use swc_ecma_ast::{VarDecl, VarDeclarator};

use super::{CaptureFreeVariables, FreeVariables};

impl CaptureFreeVariables for VarDecl {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        for decl in &self.decls {
            decl.capture_free_vars(free_vars);
        }
    }
}

impl CaptureFreeVariables for VarDeclarator {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        if let Some(init) = &self.init {
            init.capture_free_vars(free_vars);
        }

        self.name.capture_free_vars(free_vars);
    }
}

#[cfg(test)]
mod tests {
    use crate::js::capture::tests::*;

    #[test]
    fn var_assign_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = true;
            let bar = foo;
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn var_assign_free_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let bar = foo;
            "#,
        ));

        let fv_id = swc_atoms::atom!("foo");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }
}

use swc_ecma_ast::{ForHead, ForInStmt};

use super::{CaptureFreeVariables, FreeVariables};

impl CaptureFreeVariables for ForInStmt {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.left.capture_free_vars(free_vars);
        self.right.capture_free_vars(free_vars);
        self.body.capture_free_vars(free_vars);
    }
}

impl CaptureFreeVariables for ForHead {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        match self {
            ForHead::Pat(pat) => pat.capture_free_vars(free_vars),
            ForHead::UsingDecl(_) => unimplemented!("Not implementing using!"),
            ForHead::VarDecl(var_decl) => var_decl.capture_free_vars(free_vars),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::js::capture::tests::*;

    #[test]
    fn for_in_body_use_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = {};
            let buz = true;
            for (let bar in foo) {
                buz = false;
            }
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn for_in_body_use_bound_iterator_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = {};
            for (let bar in foo) {
                bar = false;
            }
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn for_in_body_use_free_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = {};
            for (let bar in foo) {
                buz = false;
            }
            "#,
        ));

        let fv_id = swc_atoms::atom!("buz");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }

    #[test]
    fn for_in_head_use_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = {};
            for (let bar in foo) {}
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn for_in() {
        let fv = compute_free_vars(parse_script(
            r#"
            for (let bar in foo) {}
            "#,
        ));

        let fv_id = swc_atoms::atom!("foo");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }
}

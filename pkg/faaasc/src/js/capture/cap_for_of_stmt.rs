use swc_ecma_ast::ForOfStmt;

use super::{CaptureFreeVariables, FreeVariables};

impl CaptureFreeVariables for ForOfStmt {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.left.capture_free_vars(free_vars);
        self.right.capture_free_vars(free_vars);
        self.body.capture_free_vars(free_vars);
    }
}

#[cfg(test)]
mod tests {
    use crate::js::capture::tests::*;

    #[test]
    fn for_in_body_use_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = [];
            let buz = true;
            for (let bar of foo) {
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
            let foo = [];
            for (let bar of foo) {
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
            let foo = [];
            for (let bar of foo) {
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
            let foo = [];
            for (let bar of foo) {}
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn for_in() {
        let fv = compute_free_vars(parse_script(
            r#"
            for (let bar of foo) {}
            "#,
        ));

        let fv_id = swc_atoms::atom!("foo");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }
}

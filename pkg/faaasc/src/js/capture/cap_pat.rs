use swc_ecma_ast::{
    ArrayPat, AssignPat, AssignPatProp, KeyValuePatProp, ObjectPat, ObjectPatProp, Pat, PropName,
    RestPat,
};

use super::{CaptureFreeVariables, FreeVariables};

impl CaptureFreeVariables for Pat {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        match self {
            // Found identity declaration expression.
            Pat::Ident(binding_ident) => free_vars.push_decl(binding_ident),
            // Otherwise recurse into substructures.
            Pat::Array(arr_pat) => arr_pat.capture_free_vars(free_vars),
            Pat::Assign(ass_pat) => ass_pat.capture_free_vars(free_vars),
            Pat::Invalid(_) => (),
            Pat::Object(obj_pat) => obj_pat.capture_free_vars(free_vars),

            // Out-of-scope patterns
            Pat::Expr(_) => unimplemented!("For-in and For-of loops are currently out of scope"),
            _ => unimplemented!("Capturing scope from this type is not implemented yet!"),
        }
    }
}

impl CaptureFreeVariables for ArrayPat {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        for elem in &self.elems {
            if let Some(elem) = elem {
                elem.capture_free_vars(free_vars)
            }
        }
    }
}

impl CaptureFreeVariables for AssignPat {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.left.capture_free_vars(free_vars);
        self.right.capture_free_vars(free_vars);
    }
}

impl CaptureFreeVariables for ObjectPat {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        for prop in &self.props {
            prop.capture_free_vars(free_vars);
        }
    }
}

impl CaptureFreeVariables for ObjectPatProp {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        match self {
            ObjectPatProp::Assign(ass_pat_prop) => ass_pat_prop.capture_free_vars(free_vars),
            ObjectPatProp::KeyValue(kv_pat_prop) => kv_pat_prop.capture_free_vars(free_vars),
            ObjectPatProp::Rest(res_pat) => res_pat.capture_free_vars(free_vars),
        }
    }
}

impl CaptureFreeVariables for AssignPatProp {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        if let Some(value) = &self.value {
            value.capture_free_vars(free_vars);
        }

        free_vars.push_decl(&self.key)
    }
}

impl CaptureFreeVariables for KeyValuePatProp {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        if let PropName::Computed(computed_prop_name) = &self.key {
            // Handle the case of:
            // ```js
            // const propname = "bar";
            // const { [propname]: blah } = foo;
            // ```
            computed_prop_name.expr.capture_free_vars(free_vars);
        }

        self.value.capture_free_vars(free_vars)
    }
}

impl CaptureFreeVariables for RestPat {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.arg.capture_free_vars(free_vars)
    }
}

#[cfg(test)]
mod tests {
    use crate::js::capture::tests::*;

    #[test]
    fn var_destructure_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = { bar: "buz" };
            let { bar } = foo;
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn var_destructure_nested_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = { bar: { buz: "buz" } };
            let { bar: { buz } } = foo;
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn var_destructure_free_self_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = { bar: "buz" };
            let { bar } = bar;
            "#,
        ));

        let fv_id = swc_atoms::atom!("bar");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }

    #[test]
    fn var_destructure_free_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = { bar: "buz" };
            let { buz } = bar;
            "#,
        ));

        let fv_id = swc_atoms::atom!("bar");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }

    #[test]
    fn var_use_destructured_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = { bar: { buz: "buz" } };
            let { bar } = foo;
            let { buz } = bar
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn var_use_rest_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = { bar: { buz: "buz" } };
            let { bar, ...buz } = foo;
            buz = {}
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn var_arr_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = [1];
            let [bar] = foo;
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn var_arr_nested_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = [[1]];
            let [[bar]] = foo;
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn var_arr_free_self_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = [1];
            let [bar] = bar;
            "#,
        ));

        let fv_id = swc_atoms::atom!("bar");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }

    #[test]
    fn var_arr_free_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = [1];
            let [buz] = bar;
            "#,
        ));

        let fv_id = swc_atoms::atom!("bar");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }

    #[test]
    fn var_prop_set_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = {};
            foo.bar = 0;
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn var_prop_set_free_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            foo.bar = 0;
            "#,
        ));

        let fv_id = swc_atoms::atom!("foo");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }

    #[test]
    fn var_computed_prop_set_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = {};
            let bar = "bar";
            foo[bar] = 0;
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn var_computed_prop_set_free_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = {}
            foo[bar] = 0;
            "#,
        ));

        let fv_id = swc_atoms::atom!("bar");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }

    #[test]
    fn var_destructure_rename_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = { bar: "buz" };
            let { bar: buz } = foo;

            let quz = buz;
            "#,
        ));

        assert!(fv.get().len() == 0);
    }
}

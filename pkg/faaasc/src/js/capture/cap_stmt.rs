use swc_ecma_ast::Stmt;

use super::{CaptureFreeVariables, FreeVariables};

impl CaptureFreeVariables for Stmt {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        match self {
            Stmt::Block(block_stmt) => block_stmt.capture_free_vars(free_vars),
            Stmt::Break(break_stmt) => break_stmt.capture_free_vars(free_vars),
            Stmt::Continue(continue_stmt) => continue_stmt.capture_free_vars(free_vars),
            Stmt::Debugger(_) => (), // Can remove this!
            Stmt::Decl(decl_stmt) => decl_stmt.capture_free_vars(free_vars),
            Stmt::DoWhile(do_while_stmt) => do_while_stmt.capture_free_vars(free_vars),
            Stmt::Empty(_) => (),
            Stmt::Expr(expr_stmt) => expr_stmt.capture_free_vars(free_vars),
            Stmt::For(for_stmt) => for_stmt.capture_free_vars(free_vars),
            Stmt::ForIn(for_in_stmt) => for_in_stmt.capture_free_vars(free_vars),
            Stmt::ForOf(for_of_stmt) => for_of_stmt.capture_free_vars(free_vars),
            Stmt::If(if_stmt) => if_stmt.capture_free_vars(free_vars),
            Stmt::Labeled(_) => unimplemented!("Not supporting labelled statements"),
            Stmt::Return(ret_stmt) => ret_stmt.capture_free_vars(free_vars),
            Stmt::Switch(switch_stmt) => switch_stmt.capture_free_vars(free_vars),
            Stmt::Throw(throw_stmt) => throw_stmt.capture_free_vars(free_vars),
            Stmt::Try(try_stmt) => try_stmt.capture_free_vars(free_vars),
            Stmt::While(while_stmt) => while_stmt.capture_free_vars(free_vars),
            Stmt::With(_) => unimplemented!("Not supporting with statements"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::js::capture::tests::*;

    #[test]
    fn ass_bound_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = 0;
            foo = 5;
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn ass_bound_var_multiple() {
        let fv = compute_free_vars(parse_script(
            r#"
            let foo = 0;
            let bar = 0;
            foo = 5;
            bar = 5;
            "#,
        ));

        assert!(fv.get().len() == 0);
    }

    #[test]
    fn ass_free_var() {
        let fv = compute_free_vars(parse_script(
            r#"
            foo = 5;
            "#,
        ));

        let fv_id = swc_atoms::atom!("foo");
        let fv_sc = swc_common::SyntaxContext::from_u32(0);

        assert!(fv.get().len() == 1);
        assert!(fv.get().contains(&(fv_id, fv_sc)));
    }

    #[test]
    fn ass_free_var_multiple() {
        let fv = compute_free_vars(parse_script(
            r#"
            foo = 5;
            bar = 0;
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

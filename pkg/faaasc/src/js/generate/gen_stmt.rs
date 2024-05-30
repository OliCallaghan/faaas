use swc_atoms::Atom;
use swc_ecma_ast::{Decl, Expr, Stmt, VarDecl, VarDeclarator};

use super::GenerateContinuation;

impl GenerateContinuation for Stmt {
    fn generate_continuation(&self) -> (Option<Atom>, Option<Atom>, Self) {
        match self {
            Stmt::Decl(decl) => {
                let (fn_cont, fn_ctx, decl) = decl.generate_continuation();

                (fn_cont, fn_ctx, Stmt::Decl(decl))
            }
            _ => (None, None, self.clone()),
        }
    }
}

impl GenerateContinuation for Decl {
    fn generate_continuation(&self) -> (Option<Atom>, Option<Atom>, Self) {
        match self {
            Decl::Var(var_decl) => {
                let (fn_cont, fn_ctx, var_decl) = var_decl.generate_continuation();
                (fn_cont, fn_ctx, Decl::Var(Box::new(var_decl)))
            }
            _ => (None, None, self.clone()),
        }
    }
}

impl GenerateContinuation for VarDecl {
    fn generate_continuation(&self) -> (Option<Atom>, Option<Atom>, Self) {
        if self.decls.len() != 1 {
            return (None, None, self.clone());
        }

        let var_decl = &self.decls[0];

        // extract the name of the identifier from the pat on var_decl.
        let ident = match &var_decl.name {
            swc_ecma_ast::Pat::Ident(ident) => ident,
            _ => return (None, None, self.clone()),
        };

        let var_decl_init = var_decl
            .init
            .as_ref()
            .map(|expr| expr.generate_continuation());

        let var_decl = VarDecl {
            span: Default::default(),
            kind: self.kind,
            declare: false,
            decls: vec![VarDeclarator {
                span: Default::default(),
                name: swc_ecma_ast::Pat::Ident(ident.clone()),
                init: var_decl_init
                    .as_ref()
                    .map(|(_, _, init)| Box::new(init.clone())),
                definite: false,
            }],
        };

        let var_decl_init_fn = var_decl_init
            .as_ref()
            .and_then(|(fn_name, _, _)| fn_name.clone());

        (var_decl_init_fn, Some(ident.sym.clone()), var_decl)
    }
}

impl GenerateContinuation for Expr {
    fn generate_continuation(&self) -> (Option<Atom>, Option<Atom>, Self) {
        match self {
            Expr::Await(await_expr) => await_expr.arg.generate_continuation(),
            Expr::Paren(paren_expr) => paren_expr.expr.generate_continuation(),
            Expr::Call(call_expr) => (
                call_expr
                    .callee
                    .clone()
                    .expr()
                    .and_then(|expr| expr.ident())
                    .map(|ident| ident.sym),
                None,
                Expr::Array(swc_ecma_ast::ArrayLit {
                    span: Default::default(),
                    elems: call_expr.args.iter().map(|arg| Some(arg.clone())).collect(),
                }),
            ),
            _ => (None, None, self.clone()),
        }
    }
}

use swc_atoms::Atom;
use swc_ecma_ast::{
    CallExpr, Callee, Decl, Expr, ExprOrSpread, MemberExpr, MemberProp, Stmt, VarDecl,
    VarDeclarator,
};

use super::{GenerateContinuation, GenerateDeclFromCtxData};

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

impl GenerateDeclFromCtxData for Stmt {
    fn generate_decl_from_ctx_data(&self) -> Self {
        match self {
            Stmt::Decl(decl) => Stmt::Decl(decl.generate_decl_from_ctx_data()),
            _ => self.clone(),
        }
    }
}

impl GenerateDeclFromCtxData for Decl {
    fn generate_decl_from_ctx_data(&self) -> Self {
        match self {
            Decl::Var(var_decl) => Decl::Var(Box::new(var_decl.generate_decl_from_ctx_data())),
            _ => self.clone(),
        }
    }
}

impl GenerateDeclFromCtxData for VarDecl {
    fn generate_decl_from_ctx_data(&self) -> Self {
        let decls = self
            .decls
            .iter()
            .map(GenerateDeclFromCtxData::generate_decl_from_ctx_data)
            .collect();

        VarDecl {
            span: self.span,
            kind: self.kind,
            declare: self.declare,
            decls,
        }
    }
}

impl GenerateDeclFromCtxData for VarDeclarator {
    fn generate_decl_from_ctx_data(&self) -> Self {
        let deserialiser_expr = Expr::Member(MemberExpr {
            span: Default::default(),
            obj: Box::new(Expr::Ident("JSON".into())),
            prop: MemberProp::Ident("parse".into()),
        });

        let ctx_data_expr = Expr::Member(MemberExpr {
            span: Default::default(),
            obj: Box::new(Expr::Ident("ctx".into())),
            prop: MemberProp::Ident("data".into()),
        });

        let init_expr = Expr::Call(CallExpr {
            span: Default::default(),
            callee: Callee::Expr(Box::new(deserialiser_expr)),
            args: vec![ExprOrSpread {
                expr: Box::new(ctx_data_expr),
                spread: None,
            }],
            type_args: None,
        });

        VarDeclarator {
            span: self.span,
            name: self.name.clone(),
            init: Some(Box::new(init_expr)),
            definite: self.definite,
        }
    }
}

use swc_ecma_ast::{Expr, ExprStmt, Lit, ParenExpr, Stmt};

pub trait Directive {
    fn is_directive(&self, directive: &str) -> bool;
}

impl Directive for Stmt {
    fn is_directive(&self, directive: &str) -> bool {
        match self {
            Stmt::Expr(expr) => expr.is_directive(directive),
            _ => false,
        }
    }
}

impl Directive for ExprStmt {
    fn is_directive(&self, directive: &str) -> bool {
        self.expr.is_directive(directive)
    }
}

impl Directive for Expr {
    fn is_directive(&self, directive: &str) -> bool {
        match self {
            Expr::Paren(paren_expr) => paren_expr.is_directive(directive),
            Expr::Lit(lit) => lit.is_directive(directive),
            _ => false,
        }
    }
}

impl Directive for ParenExpr {
    fn is_directive(&self, directive: &str) -> bool {
        self.expr.is_directive(directive)
    }
}

impl Directive for Lit {
    fn is_directive(&self, directive: &str) -> bool {
        match self {
            Lit::Str(str_lit) => str_lit.value == directive,
            _ => false,
        }
    }
}

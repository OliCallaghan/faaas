use swc_ecma_ast::{Expr, Lit, Str};

pub trait EvalLit {
    fn eval_lit_str(&self) -> Option<&Str>;
}

impl EvalLit for Expr {
    fn eval_lit_str(&self) -> Option<&Str> {
        match self {
            Expr::Paren(paren_expr) => paren_expr.expr.eval_lit_str(),
            Expr::Lit(lit_expr) => lit_expr.eval_lit_str(),
            _ => None,
        }
    }
}

impl EvalLit for Lit {
    fn eval_lit_str(&self) -> Option<&Str> {
        match self {
            Lit::Str(str_val) => Some(str_val),
            _ => None,
        }
    }
}

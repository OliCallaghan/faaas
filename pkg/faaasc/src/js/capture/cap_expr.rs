use swc_ecma_ast::{
    ArrayLit, ArrowExpr, AssignExpr, AssignTarget, AssignTargetPat, BinExpr, BlockStmtOrExpr,
    CallExpr, Callee, CondExpr, Expr, FnExpr, Function, GetterProp, KeyValueProp, MemberExpr,
    MemberProp, MethodProp, NewExpr, ObjectLit, OptCall, OptChainBase, OptChainExpr, Prop,
    PropOrSpread, SeqExpr, SetterProp, SimpleAssignTarget, SuperProp, SuperPropExpr, TaggedTpl,
    Tpl, TsInstantiation, UnaryExpr, UpdateExpr, YieldExpr,
};

use super::{CaptureFreeVariables, FreeVariables};

impl CaptureFreeVariables for Expr {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        match self {
            // Found identity expression
            Expr::Ident(ident_expr) => free_vars.push_usage(ident_expr),
            // Otherwise recurse into substructure
            Expr::Array(arr_expr) => arr_expr.capture_free_vars(free_vars),
            Expr::Arrow(arrow_expr) => arrow_expr.capture_free_vars(free_vars),
            Expr::Assign(ass_expr) => ass_expr.capture_free_vars(free_vars),
            Expr::Await(await_expr) => await_expr.arg.capture_free_vars(free_vars),
            Expr::Bin(bin_expr) => bin_expr.capture_free_vars(free_vars),
            Expr::Call(call_expr) => call_expr.capture_free_vars(free_vars),
            Expr::Cond(cond_expr) => cond_expr.capture_free_vars(free_vars),
            // TODO: Capture function definition for use later on.
            Expr::Fn(fn_expr) => fn_expr.capture_free_vars(free_vars),
            Expr::Invalid(_) => (),
            Expr::Lit(_) => (),
            Expr::Member(member_expr) => member_expr.capture_free_vars(free_vars),
            Expr::MetaProp(_) => (),
            Expr::New(new_expr) => new_expr.capture_free_vars(free_vars),
            Expr::Object(obj_expr) => obj_expr.capture_free_vars(free_vars),
            Expr::OptChain(opt_chain_expr) => opt_chain_expr.capture_free_vars(free_vars),
            Expr::Paren(paren_expr) => paren_expr.expr.capture_free_vars(free_vars),
            Expr::PrivateName(priv_name_expr) => free_vars.push_usage(&priv_name_expr.id),
            Expr::Seq(seq_expr) => seq_expr.capture_free_vars(free_vars),
            Expr::SuperProp(super_prop_expr) => super_prop_expr.capture_free_vars(free_vars),
            Expr::TaggedTpl(tagged_tpl) => tagged_tpl.capture_free_vars(free_vars),
            Expr::This(_) => (),
            Expr::Tpl(tpl) => tpl.capture_free_vars(free_vars),
            Expr::Unary(unary_expr) => unary_expr.capture_free_vars(free_vars),
            Expr::Update(update_expr) => update_expr.capture_free_vars(free_vars),
            Expr::Yield(yield_expr) => yield_expr.capture_free_vars(free_vars),
            Expr::TsAs(_) => (),
            Expr::TsNonNull(_) => (),
            Expr::TsSatisfies(_) => (),
            Expr::TsInstantiation(ts_instantiation) => {
                ts_instantiation.capture_free_vars(free_vars)
            }
            _ => unimplemented!("Capturing scope from this type is not implemented yet!"),
        }
    }
}

impl CaptureFreeVariables for ArrayLit {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        for elem in &self.elems {
            if let Some(expr_or_spread) = elem {
                expr_or_spread.expr.capture_free_vars(free_vars)
            }
        }
    }
}

impl CaptureFreeVariables for ArrowExpr {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        for param in &self.params {
            param.capture_free_vars(free_vars);
        }

        self.body.capture_free_vars(free_vars)
    }
}

impl CaptureFreeVariables for BlockStmtOrExpr {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        match self {
            BlockStmtOrExpr::BlockStmt(block_stmt) => block_stmt.capture_free_vars(free_vars),
            BlockStmtOrExpr::Expr(expr_stmt) => expr_stmt.capture_free_vars(free_vars),
        }
    }
}

impl CaptureFreeVariables for AssignExpr {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.left.capture_free_vars(free_vars);
        self.right.capture_free_vars(free_vars)
    }
}

impl CaptureFreeVariables for AssignTarget {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        match self {
            AssignTarget::Pat(ass_target_pat) => ass_target_pat.capture_free_vars(free_vars),
            AssignTarget::Simple(simple_ass_target) => {
                simple_ass_target.capture_free_vars(free_vars)
            }
        }
    }
}

impl CaptureFreeVariables for AssignTargetPat {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        match self {
            // TODO: Is this correct?
            AssignTargetPat::Array(arr_pat) => arr_pat.capture_free_vars(free_vars),
            AssignTargetPat::Object(obj_pat) => obj_pat.capture_free_vars(free_vars),
            AssignTargetPat::Invalid(_) => (),
        }
    }
}

impl CaptureFreeVariables for SimpleAssignTarget {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        match self {
            SimpleAssignTarget::Ident(ident) => free_vars.push_usage(ident),
            SimpleAssignTarget::Invalid(_) => (),
            SimpleAssignTarget::Member(member_expr) => member_expr.capture_free_vars(free_vars),
            SimpleAssignTarget::OptChain(opt_chain_expr) => {
                opt_chain_expr.capture_free_vars(free_vars)
            }
            SimpleAssignTarget::Paren(paren_expr) => paren_expr.expr.capture_free_vars(free_vars),
            _ => unimplemented!("Not implemented yet!"),
        }
    }
}

impl CaptureFreeVariables for MemberExpr {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.obj.capture_free_vars(free_vars);

        if let MemberProp::Computed(computed_member_prop) = &self.prop {
            // Catch case when
            // ```
            // const foo { bar: "buz" }
            // const prop = "bar"
            // foo[prop] = "quz"
            // ```
            computed_member_prop.expr.capture_free_vars(free_vars);
        }
    }
}

impl CaptureFreeVariables for OptChainExpr {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.base.capture_free_vars(free_vars);
    }
}

impl CaptureFreeVariables for OptChainBase {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        match self {
            OptChainBase::Call(opt_call) => opt_call.capture_free_vars(free_vars),
            OptChainBase::Member(member_expr) => member_expr.capture_free_vars(free_vars),
        }
    }
}

impl CaptureFreeVariables for OptCall {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.callee.capture_free_vars(free_vars);

        for arg in &self.args {
            arg.expr.capture_free_vars(free_vars);
        }
    }
}

impl CaptureFreeVariables for BinExpr {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.left.capture_free_vars(free_vars);
        self.right.capture_free_vars(free_vars)
    }
}

impl CaptureFreeVariables for CallExpr {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.callee.capture_free_vars(free_vars);

        for arg in &self.args {
            arg.expr.capture_free_vars(free_vars);
        }
    }
}

impl CaptureFreeVariables for Callee {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        match self {
            Callee::Expr(expr) => expr.capture_free_vars(free_vars),
            _ => unimplemented!("Only expression callee is implemented!"),
        }
    }
}

impl CaptureFreeVariables for CondExpr {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.test.capture_free_vars(free_vars);
        self.cons.capture_free_vars(free_vars);
        self.alt.capture_free_vars(free_vars)
    }
}

impl CaptureFreeVariables for FnExpr {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        if let Some(ident) = &self.ident {
            free_vars.push_decl(ident);
        }

        self.function.capture_free_vars(free_vars)
    }
}

impl CaptureFreeVariables for Function {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        for param in &self.params {
            // Capture variable declarations from function parameters.
            param.pat.capture_free_vars(free_vars);
        }

        if let Some(body) = &self.body {
            body.capture_free_vars(free_vars);
        }
    }
}

impl CaptureFreeVariables for NewExpr {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.callee.capture_free_vars(free_vars);

        if let Some(args) = &self.args {
            for arg in args {
                arg.expr.capture_free_vars(free_vars);
            }
        }
    }
}

impl CaptureFreeVariables for ObjectLit {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        for prop_or_spread in &self.props {
            prop_or_spread.capture_free_vars(free_vars);
        }
    }
}

impl CaptureFreeVariables for PropOrSpread {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        match self {
            PropOrSpread::Prop(prop) => prop.capture_free_vars(free_vars),
            PropOrSpread::Spread(spread) => spread.expr.capture_free_vars(free_vars),
        }
    }
}

impl CaptureFreeVariables for Prop {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        match self {
            Prop::Assign(_) => unimplemented!("No idea what this is used for!"),
            Prop::Getter(getter_prop) => getter_prop.capture_free_vars(free_vars),
            Prop::KeyValue(kv_prop) => kv_prop.capture_free_vars(free_vars),
            Prop::Method(method_prop) => method_prop.capture_free_vars(free_vars),
            Prop::Setter(setter_prop) => setter_prop.capture_free_vars(free_vars),
            Prop::Shorthand(shorthand_prop) => free_vars.push_usage(shorthand_prop),
        }
    }
}

impl CaptureFreeVariables for GetterProp {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        if let Some(body) = &self.body {
            body.capture_free_vars(free_vars);
        }
    }
}

impl CaptureFreeVariables for KeyValueProp {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.value.capture_free_vars(free_vars)
    }
}

impl CaptureFreeVariables for MethodProp {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.function.capture_free_vars(free_vars)
    }
}

impl CaptureFreeVariables for SetterProp {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        if let Some(this_param) = &self.this_param {
            this_param.capture_free_vars(free_vars);
        }

        self.param.capture_free_vars(free_vars);

        if let Some(body) = &self.body {
            body.capture_free_vars(free_vars);
        }
    }
}

impl CaptureFreeVariables for SeqExpr {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        for expr in &self.exprs {
            expr.capture_free_vars(free_vars);
        }
    }
}

impl CaptureFreeVariables for SuperPropExpr {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        if let SuperProp::Computed(computed_prop_name) = &self.prop {
            computed_prop_name.expr.capture_free_vars(free_vars);
        }
    }
}

impl CaptureFreeVariables for TaggedTpl {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.tag.capture_free_vars(free_vars);
        self.tpl.capture_free_vars(free_vars);
    }
}

impl CaptureFreeVariables for Tpl {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        for expr in &self.exprs {
            expr.capture_free_vars(free_vars);
        }
    }
}

impl CaptureFreeVariables for UnaryExpr {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.arg.capture_free_vars(free_vars)
    }
}

impl CaptureFreeVariables for UpdateExpr {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.arg.capture_free_vars(free_vars)
    }
}

impl CaptureFreeVariables for YieldExpr {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        if let Some(arg) = &self.arg {
            arg.capture_free_vars(free_vars)
        }
    }
}

impl CaptureFreeVariables for TsInstantiation {
    fn capture_free_vars(&self, free_vars: &mut FreeVariables) {
        self.expr.capture_free_vars(free_vars)
    }
}

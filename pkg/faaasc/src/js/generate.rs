use anyhow::Result;
use swc_atoms::Atom;
use swc_ecma_ast::{
    ArrayLit, AssignPatProp, BindingIdent, BlockStmt, CallExpr, Callee, Decl, ExportDecl, Expr,
    ExprOrSpread, FnDecl, Function, Ident, Lit, MemberExpr, MemberProp, ModuleDecl, ModuleItem,
    ObjectLit, ObjectPat, ObjectPatProp, Pat, Prop, PropOrSpread, ReturnStmt, Stmt, TsEntityName,
    TsType, TsTypeAnn, TsTypeRef, VarDecl, VarDeclarator,
};

pub mod gen_block_stmt;
pub mod gen_decl;
pub mod gen_export_decl;
pub mod gen_fn_decl;
pub mod gen_function;
pub mod gen_module;
pub mod gen_module_decl;
pub mod gen_module_item;
pub mod gen_stmt;

use super::capture::FreeVariables;

pub trait Generate {
    fn generate(&mut self) -> Result<()>;
}

pub trait GenerateWithTarget {
    fn generate(&self, gen_target: &mut Vec<Generation>) -> Result<()>;
}

pub trait GenerateWithTargetAndIdent {
    fn generate(&self, gen_target: &mut Vec<Generation>, id: Ident) -> Result<()>;
}

pub trait GenerateContinuation {
    fn generate_continuation(&self) -> (Option<Atom>, Option<Atom>, Self);
}

pub struct Generation {
    handler_name: Ident,
    handler_split_id: u32,
    handler_next: Option<(Ident, u32)>,
    stmts: Vec<Stmt>,
    free_vars: Option<FreeVariables>,
    continuation_vars: Option<FreeVariables>,
    continuation_task_ident: Option<Atom>,
    continuation_task_args_ident: Option<Atom>,
}

impl Generation {
    pub fn new(handler_name: Ident) -> Self {
        Self {
            handler_name,
            handler_split_id: 0,
            handler_next: None,
            stmts: vec![],
            free_vars: None,
            continuation_vars: None,
            continuation_task_ident: None,
            continuation_task_args_ident: None,
        }
    }

    pub fn push_stmt(&mut self, stmt: Stmt) {
        self.stmts.push(stmt)
    }

    pub fn add_continuation_vars(&mut self, continuation_vars: FreeVariables) {
        self.continuation_vars = Some(continuation_vars)
    }

    pub fn add_continuation_task_ident(&mut self, task_ident: Atom) {
        self.continuation_task_ident = Some(task_ident);
    }

    pub fn add_continuation_task_args_ident(&mut self, task_args_ident: Atom) {
        self.continuation_task_args_ident = Some(task_args_ident);
    }

    pub fn next(&mut self) -> Self {
        self.handler_next = Some((self.handler_name.clone(), self.handler_split_id + 1));

        Self {
            handler_name: self.handler_name.clone(),
            handler_split_id: self.handler_split_id + 1,
            handler_next: None,
            stmts: vec![],
            free_vars: self.continuation_vars.clone(),
            continuation_vars: None,
            continuation_task_ident: None,
            continuation_task_args_ident: None,
        }
    }

    pub fn to_module_item(&self) -> ModuleItem {
        let fn_ident_str = format!(
            "{}_{}",
            self.handler_name.sym.as_str(),
            self.handler_split_id
        );
        let fn_ident_atom = swc_atoms::Atom::new(fn_ident_str);
        let fn_ident = Ident::new(fn_ident_atom, Default::default());

        // Context
        let fn_param_ctx_ts_ann_atom = swc_atoms::atom!("TaskContext");
        let fn_param_ctx_ts_ann_id = Ident::new(fn_param_ctx_ts_ann_atom, Default::default());

        let fn_param_ctx_atom = swc_atoms::atom!("ctx");
        let fn_param_ctx_id = Ident::new(fn_param_ctx_atom, Default::default());

        let fn_param_ctx = Pat::Ident(BindingIdent {
            id: fn_param_ctx_id,
            type_ann: Some(Box::new(TsTypeAnn {
                span: Default::default(),
                type_ann: Box::new(TsType::TsTypeRef(TsTypeRef {
                    span: Default::default(),
                    type_name: TsEntityName::Ident(fn_param_ctx_ts_ann_id),
                    type_params: None,
                })),
            })),
        });

        // State
        let fn_param_state_ts_ann_atom = swc_atoms::atom!("TaskContextState");
        let fn_param_state_ts_ann_id = Ident::new(fn_param_state_ts_ann_atom, Default::default());

        let fn_param_state_atom = swc_atoms::atom!("state");
        let fn_param_state_id = Ident::new(fn_param_state_atom, Default::default());

        let fn_param_state = Pat::Ident(BindingIdent {
            id: fn_param_state_id.clone(),
            type_ann: Some(Box::new(TsTypeAnn {
                span: Default::default(),
                type_ann: Box::new(TsType::TsTypeRef(TsTypeRef {
                    span: Default::default(),
                    type_name: TsEntityName::Ident(fn_param_state_ts_ann_id),
                    type_params: None,
                })),
            })),
        });

        // Statement to redefine state
        let fv_obj_pat_props = self
            .free_vars
            .as_ref()
            .map(|fv| fv.get())
            .unwrap_or_default()
            .into_iter()
            .map(|(fv_atom, _)| Ident::new(fv_atom, Default::default()))
            .map(|fv_ident| {
                ObjectPatProp::Assign(AssignPatProp {
                    span: Default::default(),
                    key: BindingIdent {
                        id: fv_ident,
                        type_ann: None,
                    },
                    value: None,
                })
            })
            .collect::<Vec<ObjectPatProp>>();

        println!("Free Variables during export: {:?}", self.free_vars);

        let fn_stmt_define_state = Stmt::Decl(Decl::Var(Box::new(VarDecl {
            span: Default::default(),
            kind: swc_ecma_ast::VarDeclKind::Const,
            declare: false,
            decls: vec![VarDeclarator {
                span: Default::default(),
                init: Some(Box::new(Expr::Ident(fn_param_state_id))),
                name: Pat::Object(ObjectPat {
                    span: Default::default(),
                    props: fv_obj_pat_props,
                    optional: false,
                    type_ann: None,
                }),
                definite: false,
            }],
        })));

        let cv_obj_lit_props = self
            .continuation_vars
            .as_ref()
            .map(|cv| cv.get())
            .unwrap_or_default()
            .into_iter()
            .map(|(cv_atom, _)| Ident::new(cv_atom, Default::default()))
            .map(|cv_ident| PropOrSpread::Prop(Box::new(Prop::Shorthand(cv_ident))))
            .collect::<Vec<PropOrSpread>>();

        let continuation_fn_ident =
            Ident::new(swc_atoms::atom!("continuation"), Default::default());

        let continuation_fn_cap_scope = ExprOrSpread {
            spread: None,
            expr: Box::new(Expr::Object(ObjectLit {
                span: Default::default(),
                props: cv_obj_lit_props,
            })),
        };
        let continuation_fn_task_ident = ExprOrSpread {
            spread: None,
            expr: Box::new(Expr::Member(MemberExpr {
                span: Default::default(),
                obj: Box::new(Expr::Ident(Ident::new(
                    self.continuation_task_ident
                        .as_ref()
                        .unwrap_or(&swc_atoms::atom!("undefined"))
                        .clone(),
                    Default::default(),
                ))),
                prop: MemberProp::Ident(Ident::new(
                    swc_atoms::atom!("continuation"),
                    Default::default(),
                )),
            })),
        };

        let (handler_next_ident, handler_next_split_id) = self
            .handler_next
            .clone()
            .unwrap_or_else(|| (fn_ident.clone(), 0));
        let handler_next_atom = format!(
            "{}_{}",
            handler_next_ident.sym.as_str(),
            handler_next_split_id
        );

        let continuation_fn_args = ExprOrSpread {
            spread: None,
            expr: Box::new(Expr::Array(ArrayLit {
                span: Default::default(),
                elems: vec![
                    Some(ExprOrSpread {
                        spread: None,
                        expr: Box::new(Expr::Lit(Lit::Str(handler_next_atom.into()))),
                    }),
                    Some(ExprOrSpread {
                        spread: None,
                        expr: Box::new(Expr::Lit(Lit::Str(
                            self.continuation_task_args_ident
                                .as_ref()
                                .unwrap_or(&swc_atoms::atom!("undefined"))
                                .clone()
                                .into(),
                        ))),
                    }),
                    Some(ExprOrSpread {
                        spread: Some(Default::default()),
                        expr: Box::new(Expr::Ident(Ident::new(
                            self.continuation_task_args_ident
                                .as_ref()
                                .unwrap_or(&swc_atoms::atom!("undefined"))
                                .clone(),
                            Default::default(),
                        ))),
                    }),
                ],
            })),
        };

        let fn_ret_call_cont_state = Expr::Call(CallExpr {
            span: Default::default(),
            callee: Callee::Expr(Box::new(Expr::Ident(continuation_fn_ident))),
            args: vec![
                continuation_fn_task_ident,
                continuation_fn_args,
                continuation_fn_cap_scope,
            ],
            type_args: None,
        });

        let fn_ret_cont_state = Stmt::Return(ReturnStmt {
            span: Default::default(),
            arg: Some(Box::new(fn_ret_call_cont_state)),
        });

        let stmts = [fn_stmt_define_state]
            .into_iter()
            .chain(self.stmts.clone().into_iter())
            .chain([fn_ret_cont_state])
            .collect();

        let function = Function {
            span: Default::default(),
            params: vec![fn_param_ctx.into(), fn_param_state.into()],
            decorators: vec![],
            body: Some(BlockStmt {
                span: Default::default(),
                stmts,
            }),
            is_async: true,
            is_generator: false,
            type_params: None,
            return_type: None,
        };

        ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
            span: Default::default(),
            decl: Decl::Fn(FnDecl {
                ident: fn_ident,
                function: Box::new(function),
                declare: false,
            }),
        }))
    }
}

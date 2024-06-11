use anyhow::Result;
use swc_ecma_ast::{
    BindingIdent, CallExpr, Callee, Decl, ExportDecl, Expr, ExprOrSpread, ImportDecl,
    ImportNamedSpecifier, ImportSpecifier, Module, ModuleDecl, ModuleItem, ObjectLit, Pat, Prop,
    PropOrSpread, VarDecl, VarDeclKind, VarDeclarator,
};

use super::{Generate, GenerateWithTarget, Generation};

impl Generate for Module {
    fn generate(&mut self, adaptor: &str) -> Result<()> {
        let mut target = Vec::new();

        for item in &self.body {
            item.generate(&mut target)?;
        }

        for gen in &target {
            self.body.push(gen.to_module_item());
        }

        self.body.insert(0, generate_import_decl());
        self.body.insert(0, generate_import_adaptor_decl(adaptor));
        self.body.push(generate_entrypoint(&target));

        Ok(())
    }
}

fn generate_import_decl() -> ModuleItem {
    ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: Default::default(),
        specifiers: vec![
            ImportSpecifier::Named(ImportNamedSpecifier {
                span: Default::default(),
                local: "continuation".into(),
                imported: None,
                is_type_only: false,
            }),
            ImportSpecifier::Named(ImportNamedSpecifier {
                span: Default::default(),
                local: "FaaascInternalState".into(),
                imported: None,
                is_type_only: true,
            }),
            ImportSpecifier::Named(ImportNamedSpecifier {
                span: Default::default(),
                local: "FaaascInternalContext".into(),
                imported: None,
                is_type_only: true,
            }),
        ],
        src: Box::new("@faaas/faaasc".into()),
        type_only: Default::default(),
        with: Default::default(),
        phase: Default::default(),
    }))
}

fn generate_import_adaptor_decl(adaptor: &str) -> ModuleItem {
    ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: Default::default(),
        specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
            span: Default::default(),
            local: "buildEntrypoint".into(),
            imported: None,
            is_type_only: false,
        })],
        src: Box::new(adaptor.into()),
        type_only: Default::default(),
        with: Default::default(),
        phase: Default::default(),
    }))
}

fn generate_entrypoint(gens: &Vec<Generation>) -> ModuleItem {
    let handlers = Expr::Object(ObjectLit {
        span: Default::default(),
        props: gens
            .iter()
            .map(|gen| PropOrSpread::Prop(Box::new(Prop::Shorthand(gen.to_module_item_ident()))))
            .chain([PropOrSpread::Prop(Box::new(Prop::Shorthand(
                "handler".into(),
            )))])
            .collect(),
    });

    let entrypoint = Expr::Call(CallExpr {
        span: Default::default(),
        callee: Callee::Expr(Box::new(Expr::Ident("buildEntrypoint".into()))),
        args: vec![ExprOrSpread {
            spread: None,
            expr: Box::new(handlers),
        }],
        type_args: None,
    });

    ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
        span: Default::default(),
        decl: Decl::Var(Box::new(VarDecl {
            span: Default::default(),
            kind: VarDeclKind::Const,
            declare: false,
            decls: vec![VarDeclarator {
                span: Default::default(),
                name: Pat::Ident(BindingIdent {
                    id: "entrypoint".into(),
                    type_ann: None,
                }),
                init: Some(Box::new(entrypoint)),
                definite: false,
            }],
        })),
    }))
}

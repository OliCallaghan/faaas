use anyhow::Result;
use swc_ecma_ast::{
    ImportDecl, ImportNamedSpecifier, ImportSpecifier, Module, ModuleDecl, ModuleItem,
};

use super::{Generate, GenerateWithTarget};

impl Generate for Module {
    fn generate(&mut self) -> Result<()> {
        let mut target = Vec::new();

        for item in &self.body {
            item.generate(&mut target)?;
        }

        for gen in target {
            self.body.push(gen.to_module_item());
        }

        self.body.insert(0, generate_import_decl());

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
                local: "TaskContextState".into(),
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

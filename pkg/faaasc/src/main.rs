use std::{
    io::{Read, Write},
    path::Path,
};

use anyhow::Result;

use clap::Parser as ClapParser;
use clio::{Input, Output};
use graphviz_rust::{
    cmd::{CommandArg, Format},
    exec,
    printer::PrinterContext,
};
use swc_common::{
    errors::{ColorConfig, Handler},
    sync::Lrc,
    SourceMap,
};
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
use swc_ecma_parser::{lexer::Lexer, Capturing, Parser, StringInput, Syntax};

mod js;

use js::generate::Generate;
use js::graph::ToGraphvizGraph;

#[derive(ClapParser)]
#[clap(name = "faaasc")]
struct Args {
    #[clap(value_parser, default_value = "-")]
    input: Input,

    #[clap(value_parser, default_value = "-")]
    output: Output,
}

fn main() -> Result<()> {
    let mut args = Args::parse();

    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let fm = cm
        .load_file(args.input.path())
        .expect("failed to load file");

    let lexer = Lexer::new(
        Syntax::Typescript(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let capturing = Capturing::new(lexer);

    let mut parser = Parser::new_from(capturing);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    let mut m = parser
        .parse_typescript_module()
        .map_err(|e| e.into_diagnostic(&handler).emit())
        .expect("Failed to parse module.");

    exec(
        m.to_graph(),
        &mut PrinterContext::default(),
        vec![
            Format::Pdf.into(),
            CommandArg::Output("assets/module.pdf".to_string()),
        ],
    )?;

    exec(
        m.to_graph(),
        &mut PrinterContext::default(),
        vec![
            Format::Png.into(),
            CommandArg::Output("assets/module.png".to_string()),
        ],
    )?;

    m.generate()?;

    let mut buf = vec![];

    let mut emitter = Emitter {
        cfg: Default::default(),
        cm: cm.clone(),
        comments: None,
        wr: JsWriter::new(cm, "\n", &mut buf, None),
    };

    emitter.emit_module(&m)?;

    args.output.write_all(&buf)?;

    Ok(())
}

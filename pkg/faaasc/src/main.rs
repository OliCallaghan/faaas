use std::{env, io::Read};

use clap::Parser;
use clio::{ClioPath, Input, Output};
use swc_common::sync::Lrc;
use swc_common::SourceMap;
use swc_ecma_parser::{parse_file_as_module, Syntax, TsConfig};

#[derive(Parser, Debug)]
#[command(name = "faaasc")]
#[command(version, about, long_about = None)]
struct FaaascArgs {
    #[clap(value_parser, default_value = "-")]
    input: Input,

    #[clap(long, short, value_parser, default_value = "-")]
    output: Output,

    #[clap(long, short, value_parser = clap::value_parser!(ClioPath).exists().is_dir(), default_value = ".")]
    log_dir: ClioPath,
}

fn main() -> Result<(), std::io::Error> {
    const DEFAULT_ENTRYPOINT_SIZE: u64 = 512;

    let cwd = env::current_dir();

    let mut args = FaaascArgs::parse();

    let entrypoint_size = args.input.len().unwrap_or(DEFAULT_ENTRYPOINT_SIZE);
    let mut entrypoint = String::with_capacity(entrypoint_size as usize);

    args.input.read_to_string(&mut entrypoint)?;

    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.load_file(args.input.path())?;
    let syntax = Syntax::Typescript(TsConfig::default());

    let mut recovered_errors = vec![];

    let entrypoint_mod = parse_file_as_module(
        &fm,
        syntax,
        swc_ecma_ast::EsVersion::Es2022,
        None,
        &mut recovered_errors,
    )
    .unwrap();

    println!("Compiling {}", cwd.unwrap().to_str().unwrap());
    println!("AST: {:#?}", entrypoint_mod.body);

    Ok(())
}

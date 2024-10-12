mod cores;
mod infras;

mod imports_handler;
mod module;
mod modules;

use anyhow::Result;
use inkwell::context::Context;
use std::{fs::read_to_string, process::exit};

const MAIN_MODULE_NAME: &str = "main";
const OUTPUT_PATH_NAME: &str = "output.ll";

fn main() {
    if let Err(e) = start() {
        eprint!("Error: {e}");
        exit(1);
    }

    println!("compiled successfully!!!");
}

fn start() -> Result<()> {
    let source_path = "examples/global.py";
    let source = read_to_string(source_path)?;

    let error_reporter = infras::ErrorReporter::default();

    let parser = infras::Parser::new(&source, source_path);

    let context = Context::create();
    let output_module = context.create_module(MAIN_MODULE_NAME);
    let codegen = infras::CodeGenerator::new(&context, output_module, &OUTPUT_PATH_NAME);

    let compiler = cores::Compiler::new(parser, codegen, error_reporter);
    compiler.compile()
}

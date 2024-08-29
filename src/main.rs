mod code_gen;
mod compiler;
mod modules;
mod parser;

use anyhow::Result;
use code_gen::CodeGenerator;
use compiler::Compiler;
use inkwell::context::Context;
use std::fs::read_to_string;

const MAIN_MODULE_NAME: &str = "main";

const OUTPUT_PATH_NAME: &str = "output.ll";

fn main() {
    if let Err(e) = start() {
        eprint!("Error: {e}");
        std::process::exit(1);
    }
}

fn start() -> Result<()> {
    let source_path = "examples/import_modules.py";
    let source = read_to_string(source_path)?;

    let parser = parser::Parser::new(&source, source_path);
    let module_ast = parser.parse()?;

    println!("{:#?}", module_ast);

    let context = Context::create();
    let output_module = context.create_module(MAIN_MODULE_NAME);

    // let code_generator = CodeGenerator::new(&context);

    let compiler = Compiler::new(parser, &context, output_module, OUTPUT_PATH_NAME);
    compiler.compile()?;

    Ok(())
}

use anyhow::Result;
use crate::{code_gen::CodeGenerator, modules::PythonModule, parser::Parser};
use inkwell::{context::Context, module::Module};
use rustpython_parser::ast::{fold::Foldable, ModModule, Stmt};

pub struct Compiler<'ctx, 'a> {
    parser: Parser<'a>,

    context: &'ctx Context,
    output_module: Module<'ctx>,

    modules: Vec<Module<'ctx>>,

    output_path: &'a str,
}

impl<'ctx, 'a> Compiler<'ctx, 'a> {
    pub fn new(
        parser: Parser<'a>,
        context: &'ctx Context,
        output_module: Module<'ctx>,
        output_path: &'a str,
    ) -> Self {
        Self {
            parser,
            context,
            output_module,
            output_path,
            modules: vec![],
        }
    }

    pub fn compile(&self) -> Result<()> {
        let main_module_ast = self.parser.parse()?;

        self.compile_module(&main_module_ast)?;
        self.print_to_file()
    }

    pub fn compile_module(&self, module: &ModModule) -> Result<()> {
        for stmt in module.body.iter() {
            match stmt {
                Stmt::Import(import_stmt) => {
                    todo!("implement import statement");
                }
                Stmt::AnnAssign(typed_assign) => {
                    todo!("implement annotation assignment, Global variable");
                }
                Stmt::FunctionDef(func) => {
                    todo!("implement function definition");
                }
                Stmt::ClassDef(class) => {
                    todo!("implement class definition");
                }
                s => {
                    eprintln!("Error: unsupported top level statement: {s:?}")
                }
            }
        }
        Ok(())
    }

    fn print_to_file(&self) -> Result<()> {
        self.output_module.print_to_file(self.output_path)
            .map_err(|e| format!("Error: {:?}", e))
            .map_err(anyhow::Error::msg)
    }
}

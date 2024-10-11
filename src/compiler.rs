use crate::parser::Parser;
use anyhow::Result;
use inkwell::{context::Context, module::Module};
use rustpython_parser::ast::{ModModule, Stmt, StmtAnnAssign, StmtClassDef, StmtFunctionDef};

pub struct Compiler<'ctx, 'a> {
    parser: Parser<'a>,

    context: &'ctx Context,
    module: Module<'ctx>,

    output_path: &'a str,
}

impl<'ctx, 'a> Compiler<'ctx, 'a> {
    pub fn new(
        parser: Parser<'a>,
        context: &'ctx Context,
        module: Module<'ctx>,
        output_path: &'a str,
    ) -> Self {
        Self {
            parser,
            context,
            module,
            output_path,
        }
    }

    pub fn compile(&self) -> Result<()> {
        let main_module_ast = self.parser.parse()?;
        self.compile_module(&main_module_ast)?;
        self.print_to_file()?;
        Ok(())
    }

    pub fn compile_module(&self, module: &ModModule) -> Result<()> {
        for stmt in module.body.iter() {
            match stmt {
                Stmt::Import(import_stmt) => {
                    let imported_module = todo!("should return a imported module ast");
                    self.compile_module(imported_module)?;
                }
                Stmt::AnnAssign(typed_assignment) => {
                    self.compile_global_variable(typed_assignment)?;
                }
                Stmt::Assign(_) => {
                    return Err(anyhow::anyhow!("Assign statement is not supported"));
                }
                Stmt::FunctionDef(func) => {
                    self.compile_func_def(func)?;
                }
                Stmt::ClassDef(class) => {
                    self.compile_class_def(class)?;
                }
                s => {
                    eprintln!("Error: unsupported top level statement: {s:?}")
                }
            }
        }
        Ok(())
    }

    pub fn compile_global_variable(&self, typed_assignment_stmt: &StmtAnnAssign) -> Result<()> {
        todo!()
    }

    pub fn compile_func_def(&self, func_def_stmt: &StmtFunctionDef) -> Result<()> {
        todo!()
    }

    pub fn compile_class_def(&self, class_def_stmt: &StmtClassDef) -> Result<()> {
        todo!()
    }

    fn print_to_file(&self) -> Result<()> {
        self.module
            .print_to_file(self.output_path)
            .map_err(|e| format!("Error: {:?}", e))
            .map_err(anyhow::Error::msg)
    }
}

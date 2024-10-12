use super::{CodeGenerator, ErrorReporter, Parser};
use crate::cores::PyLitValue;
use anyhow::Result;
use rustpython_parser::ast::{
    Expr, ExprConstant, ExprName, ModModule, Stmt, StmtAnnAssign, StmtClassDef, StmtFunctionDef,
};

pub struct Compiler<P, G, R>
where
    P: Parser,
    G: CodeGenerator,
    R: ErrorReporter,
{
    codegen: G,
    parser: P,
    error_reporter: R,
}

impl<P, G, R> Compiler<P, G, R>
where
    P: Parser,
    G: CodeGenerator,
    R: ErrorReporter,
{
    pub fn new(parser: P, codegen: G, error_reporter: R) -> Self {
        Self {
            codegen,
            error_reporter,
            parser,
        }
    }

    fn print_to_file(&self) -> Result<()> {
        self.codegen.print_to_file()
    }

    pub fn compile(&self) -> Result<()> {
        let module = self.parser.parse()?;
        self.compile_module(&module)?;
        self.print_to_file()?;
        Ok(())
    }

    pub fn compile_module(&self, module: &ModModule) -> Result<()> {
        for stmt in module.body.iter() {
            match stmt {
                // Stmt::Import(import_stmt) => {
                //     let imported_module = todo!("should return a imported module ast");
                //     self.compile_module(imported_module)?;
                // }
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

    fn compile_global_variable(&self, typed_assignment: &StmtAnnAssign) -> Result<()> {
        let Some(value) = &typed_assignment.value else {
            return Err(anyhow::anyhow!(
                "value of global variable needed to be assign"
            ));
        };

        let Expr::Name(ExprName { id, range, .. }) = &*typed_assignment.target else {
            return Err(anyhow::anyhow!("unexpected error!!!"));
        };

        let Expr::Constant(ExprConstant { value, range, .. }) = &**value else {
            return Err(anyhow::anyhow!("unexpected error!!!"));
        };

        let lit = PyLitValue::try_new(value)?;

        self.codegen.add_global_variable(id, lit)
    }

    fn compile_func_def(&self, func: &StmtFunctionDef) -> Result<()> {
        todo!()
    }

    fn compile_class_def(&self, class: &StmtClassDef) -> Result<()> {
        todo!()
    }
}

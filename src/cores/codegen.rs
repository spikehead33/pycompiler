use super::literals::PyLitValue;
use anyhow::Result;
use rustpython_parser::ast::StmtFunctionDef;

pub trait CodeGenerator {
    fn add_function_def(&self, func: &StmtFunctionDef) -> Result<()>;
    fn add_global_variable(&self, var: &str, lit_val: PyLitValue) -> Result<()>;
    fn print_to_file(&self) -> Result<()>;
}

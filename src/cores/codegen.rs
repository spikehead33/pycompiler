use anyhow::Result;
use super::literals::PyLitValue;

pub trait CodeGenerator {
    fn add_global_variable(&self, var: &str, lit_val: PyLitValue) -> Result<()>;
    fn print_to_file(&self, output_path: &str) -> Result<()>;
}
use anyhow::Result;
use inkwell::{builder::Builder, context::Context, module::Module};

use crate::cores;

pub struct CodeGenerator<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

    output_path: &'static str,
}

impl<'ctx> CodeGenerator<'ctx> {
    pub fn new(context: &'ctx Context, module: Module<'ctx>, output_path: &'static str) -> Self {
        let builder = context.create_builder();
        Self {
            context,
            module,
            builder,
            output_path,
        }
    }
}

impl<'ctx> cores::CodeGenerator for CodeGenerator<'ctx> {
    fn print_to_file(&self) -> Result<()> {
        self.module
            .print_to_file(self.output_path)
            .map_err(|e| format!("Error: {:?}", e))
            .map_err(anyhow::Error::msg)
    }

    fn add_global_variable(&self, var: &str, lit_val: cores::PyLitValue) -> Result<()> {
        match lit_val {
            cores::PyLitValue::Integer(val) => {
                let i32_type = self.context.i32_type();
                let i32_val = i32_type.const_int(val as u64, false);
                let global_var = self.module.add_global(i32_type, None, var);
                global_var.set_initializer(&i32_val);
            }
            cores::PyLitValue::Float(val) => {
                let f64_type = self.context.f64_type();
                let f64_val = f64_type.const_float(val);
                let global_var = self.module.add_global(f64_type, None, var);
                global_var.set_initializer(&f64_val);
            }
            cores::PyLitValue::String(val) => {
                todo!("not implemented yet");
                // let str_type = self.context.i8_type().array_type(val.len() as u32);
                // let str_val = str_type.const_string(val.as_bytes(), false);
                // self.module.add_global(str_type, None, var);
            }
            cores::PyLitValue::Bool(val) => {
                let i1_type = self.context.bool_type();
                let i1_val = i1_type.const_int(val as u64, false);
                let global_var = self.module.add_global(i1_type, None, var);
                global_var.set_initializer(&i1_val);
            }
        }
        Ok(())
    }

    fn add_function_def(&self) -> Result<()> {
        todo!()
    }
}

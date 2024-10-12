use std::path::PathBuf;

use inkwell::module::Module;
use rustpython_parser::ast::ModModule;

pub struct PythonModule<'ctx> {
    name: String,

    source_path: PathBuf,

    module_ast: ModModule,

    llvm_module: Module<'ctx>,
}

impl<'ctx> PythonModule<'ctx> {
    pub fn new() -> Self {
        todo!()
    }
}

use inkwell::module::Module;

pub struct PythonModule<'ctx, 'a> {
    source_path: &'a str,
    source: &'a str,

    module: Module<'ctx>,
}

impl<'ctx, 'a> PythonModule<'ctx, 'a> {
    pub fn new(source_path: &'a str, source: &'a str, module: Module<'ctx>) -> Self {
        Self {
            source_path,
            source,
            module,
        }
    }
}

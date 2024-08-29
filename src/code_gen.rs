use inkwell::{builder::Builder, context::Context, module::Module};

pub struct CodeGenerator<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    modules: Vec<Module<'ctx>>,
}

impl<'ctx> CodeGenerator<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let builder = context.create_builder();
        Self {
            context,
            builder,
            modules: vec![],
        }
    }
}

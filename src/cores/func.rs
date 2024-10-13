use inkwell::{context::Context, types::FunctionType};

pub struct Function<'ctx> {
    context: &'ctx Context,
}
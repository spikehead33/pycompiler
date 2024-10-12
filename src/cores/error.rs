use rustpython_parser::text_size::TextRange;

pub trait ErrorReporter {
    fn add_error(&mut self, error: CompilationError);
    fn report(&self);
}

#[derive(Debug)]
pub struct CompilationError {
    range: TextRange,
    msg: String,
    kind: CompilationErrorKind,
}

impl CompilationError {
    pub fn new(msg: String, range: TextRange, kind: CompilationErrorKind) -> Self {
        Self { msg, range, kind }
    }
}

#[derive(Debug)]
pub enum CompilationErrorKind {
    User,
    Compiler,
}

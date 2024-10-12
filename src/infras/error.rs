use crate::cores::{self, CompilationError};

#[derive(Default)]
pub struct ErrorReporter {
    errors: Vec<CompilationError>,
}

impl cores::ErrorReporter for ErrorReporter {
    fn add_error(&mut self, error: CompilationError) {
        self.errors.push(error);
    }

    fn report(&self) {
        if self.errors.is_empty() {
            return;
        }

        for error in self.errors.iter() {
            println!("Error: {:?}", error);
        }
    }
}

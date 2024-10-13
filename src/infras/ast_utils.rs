use anyhow::Result;
use rustpython_parser::ast::{Expr, ExprName, StmtAnnAssign};

pub trait AstNodeExtractor {
    type Output;
    fn extract_from_expr(&self) -> Result<Self::Output>;
}

impl AstNodeExtractor for &StmtAnnAssign {
    type Output = ExprName;

    fn extract_from_expr(&self) -> Result<Self::Output> {
        todo!()
    }
}

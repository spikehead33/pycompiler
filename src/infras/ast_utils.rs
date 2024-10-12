use anyhow::Result;
use rustpython_parser::ast::{Expr, ExprName, StmtAnnAssign};

pub trait AstNodeExtractor {
    type AstType;
    type Output;
    fn extract_from_expr(&self, ast: Self::AstType) -> Result<Self::Output>;
}

impl AstNodeExtractor for &StmtAnnAssign {
    type AstType = StmtAnnAssign;
    type Output = ExprName;

    fn extract_from_expr(&self, ast: Self::AstType) -> Result<Self::Output> {
        todo!()
    }
}

use anyhow::Result;
use rustpython_parser::ast::ModModule;

pub trait Parser {
    fn parse(&self) -> Result<ModModule>;
}
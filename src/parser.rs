use core::panic;

use anyhow::Result;
use rustpython_parser::{
    ast::{Mod, ModModule},
    parse,
};

pub struct Parser<'a> {
    source: &'a str,
    source_path: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, source_path: &'a str) -> Self {
        Self {
            source,
            source_path,
        }
    }

    pub fn parse(&self) -> Result<ModModule> {
        let m = parse(
            self.source,
            rustpython_parser::Mode::Module,
            &self.source_path,
        )?;
        let Mod::Module(m) = m else {
            panic!("expected a module")
        };
        Ok(m)
    }
}

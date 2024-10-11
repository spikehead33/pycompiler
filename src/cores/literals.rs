use rustpython_parser::ast::{Constant, ExprConstant};
use num_traits::ToPrimitive;
use anyhow::Result;

pub enum PyLitValue {
    Integer(i32),
    Float(f64),
    Bool(bool),
    String(String),
}

impl PyLitValue {
    pub fn try_new(con: &Constant) -> Result<Self> {
        match con {
            Constant::Int(i) => {
                let int = i.to_i32().ok_or_else(|| anyhow::anyhow!("integer out of range"))?;
                Ok(PyLitValue::Integer(int))
            },
            Constant::Float(f) => Ok(PyLitValue::Float(*f)),
            Constant::Bool(b) => Ok(PyLitValue::Bool(*b)),
            _ => Err(anyhow::anyhow!("unsupported literal")),
        }
    }
}
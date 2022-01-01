use crate::{address::CellAddress, value::Value};

use super::Evaluate;

#[derive(Clone, Debug)]
pub enum Expression {
    Literal(Value),
    Reference(CellAddress),
}

impl Evaluate for Expression {
    fn evaluate(&self) -> Value {
        match self {
            Self::Literal(value) => value.clone(),
            Self::Reference(_reference) => todo!(),
        }
    }
}

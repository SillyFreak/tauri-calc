//! Formulas are a parsed representation of what is put into a cell.
//! For example, if `=1` is put into a cell, the formula consists of a (very simple) mathematical expression.
//! If `1` is put into a cell, the "formula" is a literal value

pub mod expression;

use crate::value::Value;

use self::expression::Expression;

pub trait Evaluate {
    fn evaluate(&self) -> Value;
}

#[derive(Clone, Debug)]
pub enum Formula {
    Literal(Value),
    Formula(Expression),
}

impl Default for Formula {
    fn default() -> Self {
        Self::Literal(Value::default())
    }
}

impl Evaluate for Formula {
    fn evaluate(&self) -> Value {
        match self {
            Self::Literal(value) => value.clone(),
            Self::Formula(expression) => expression.evaluate(),
        }
    }
}

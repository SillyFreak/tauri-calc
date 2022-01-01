//! Formulas are a parsed representation of what is put into a cell.
//! For example, if `=1` is put into a cell, the formula consists of a (very simple) mathematical expression.
//! If `1` is put into a cell, the "formula" is a literal value

mod error;
pub mod expression;

use nom::Finish;
use std::str::FromStr;

use crate::parser::parse_cell;
use crate::sheet::Sheet;
use crate::value::Value;

use self::expression::Expression;

pub use error::FormulaError;

pub trait Evaluate {
    fn evaluate(&self, context: &Sheet) -> Value;
}

#[derive(Clone, Debug)]
pub enum Formula {
    Literal(Value),
    Formula(Expression),
}

impl FromStr for Formula {
    type Err = FormulaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rest, formula) = parse_cell(s).finish().map_err(|_| FormulaError::Invalid)?;
        if !rest.is_empty() {
            Err(FormulaError::Invalid)?;
        }

        Ok(formula)
    }
}

impl Default for Formula {
    fn default() -> Self {
        Self::Literal(Value::default())
    }
}

impl Evaluate for Formula {
    fn evaluate(&self, context: &Sheet) -> Value {
        match self {
            Self::Literal(value) => value.clone(),
            Self::Formula(expression) => expression.evaluate(context),
        }
    }
}

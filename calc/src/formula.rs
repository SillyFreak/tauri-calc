//! Formulas are a parsed representation of what is put into a cell.
//! For example, if `=1` is put into a cell, the formula consists of a (very simple) mathematical expression.
//! If `1` is put into a cell, the "formula" is a literal value

pub mod expression;

use std::str::FromStr;

use crate::address::CellAddress;
use crate::parser::{parse_cell_complete, ParseFormulaError};
use crate::sheet::Sheet;
use crate::value::Value;

use self::expression::Expression;

pub trait Evaluate {
    fn visit_dependecies<F: FnMut(CellAddress)>(&self, visitor: &mut F);

    fn evaluate(&self, context: &Sheet) -> Value;
}

#[derive(Clone, Debug)]
pub enum Formula {
    Literal(Value),
    Formula(Expression),
}

impl FromStr for Formula {
    type Err = ParseFormulaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_cell_complete(s)
    }
}

impl Default for Formula {
    fn default() -> Self {
        Self::Literal(Value::default())
    }
}

impl Evaluate for Formula {
    fn visit_dependecies<F: FnMut(CellAddress)>(&self, visitor: &mut F) {
        match self {
            Self::Literal(_value) => {}
            Self::Formula(expression) => expression.visit_dependecies(visitor),
        }
    }

    fn evaluate(&self, context: &Sheet) -> Value {
        match self {
            Self::Literal(value) => value.clone(),
            Self::Formula(expression) => expression.evaluate(context),
        }
    }
}

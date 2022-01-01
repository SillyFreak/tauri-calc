mod error;

use nom::Finish;

use crate::formula::{Evaluate, Formula};
use crate::parser::parse_cell;
use crate::value::Value;

pub use error::{CellInputError, CellInputErrorKind};

#[derive(Clone, Debug, Default)]
pub struct Cell {
    input: String,
    formula: Formula,
    value: Value,
}

impl Cell {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_formula(&mut self, input: String) -> Result<(), CellInputError> {
        let (rest, formula) = parse_cell(&input).finish().map_err(|_| CellInputError {
            kind: CellInputErrorKind::InvalidFormula,
        })?;
        if !rest.is_empty() {
            Err(CellInputError {
                kind: CellInputErrorKind::InvalidFormula,
            })?;
        }

        let value = formula.evaluate();

        self.input = input;
        self.formula = formula;
        self.value = value;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::value::Value;

    #[test]
    fn test_empty() {
        let mut cell = Cell::new();
        cell.set_formula("".to_string()).expect("invalid formula");
        assert_eq!(cell.value, Value::Empty);
    }

    #[test]
    fn test_literal_number() {
        let mut cell = Cell::new();
        cell.set_formula("1".to_string()).expect("invalid formula");
        assert_eq!(cell.value, Value::Number("1".parse().unwrap()));
    }

    #[test]
    fn test_literal_string() {
        let mut cell = Cell::new();
        cell.set_formula("\"foo\"".to_string())
            .expect("invalid formula");
        assert_eq!(cell.value, Value::String("foo".to_string()));
    }

    #[test]
    fn test_number_formula() {
        let mut cell = Cell::new();
        cell.set_formula("=1".to_string()).expect("invalid formula");
        assert_eq!(cell.value, Value::Number("1".parse().unwrap()));
    }

    #[test]
    fn test_string_formula() {
        let mut cell = Cell::new();
        cell.set_formula("=\"foo\"".to_string())
            .expect("invalid formula");
        assert_eq!(cell.value, Value::String("foo".to_string()));
    }
}

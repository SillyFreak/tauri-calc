//! Cell values are the result of evaluating what is put into a cell.
//! For example, if `=1+1` is put into a cell, although that is a formula,
//! the *value* of the cell would be two.

use std::fmt;

use bigdecimal::BigDecimal;

/// The value of a cell
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    /// empty cells have this value
    Empty,
    /// the value of the cell is a number
    Number(BigDecimal),
    /// the value of the cell is a string
    String(String),
    // TODO errors
}

impl Default for Value {
    fn default() -> Self {
        Value::Empty
    }
}

impl From<Option<&Value>> for Value {
    fn from(value: Option<&Value>) -> Self {
        value.map_or(Value::Empty, Value::clone)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty => write!(f, ""),
            Self::Number(value) => write!(f, "{}", value),
            Self::String(value) => write!(f, "{:?}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        assert_eq!(format!("{}", Value::Empty), "");
        assert_eq!(format!("{}", Value::Number("1".parse().unwrap())), "1");
        assert_eq!(format!("{}", Value::String("foo".to_string())), "\"foo\"");
    }
}

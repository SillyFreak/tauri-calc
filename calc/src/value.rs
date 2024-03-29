//! Cell values are the result of evaluating what is put into a cell.
//! For example, if `=1+1` is put into a cell, although that is a formula,
//! the *value* of the cell would be two.

use std::fmt;

use bigdecimal::BigDecimal;
use serde::{Serialize, Serializer};

/// The value of a cell
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum Value {
    /// empty cells have this value
    Empty,
    /// the value of the cell is a number
    #[serde(serialize_with = "serialize_bigdecimal")]
    Number(BigDecimal),
    /// the value of the cell is a string
    String(String),
    /// any kind of error
    Error(Error),
}

fn serialize_bigdecimal<S>(number: &BigDecimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_newtype_struct("$tauri_calc::bigdecimal", &number.to_string())
}

/// The value of a cell
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Error {
    /// a value could not be interpreted as a certain type as necessary
    Type,
    /// a nonexistent identifier was used
    Undefined,
}

impl Value {
    pub fn as_number(&self) -> Result<&BigDecimal, Error> {
        match self {
            Self::Number(value) => Ok(value),
            Self::Error(error) => Err(*error),
            _ => Err(Error::Type),
        }
    }

    pub fn as_string(&self) -> Result<&str, Error> {
        match self {
            Self::String(value) => Ok(value),
            Self::Error(error) => Err(*error),
            _ => Err(Error::Type),
        }
    }
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
            Self::Error(error) => write!(f, "{}", error),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Type => write!(f, "#TYPE"),
            Self::Undefined => write!(f, "#UNDEFINED"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        assert_eq!(format!("{}", Value::Empty), "");
        assert_eq!(format!("{}", Value::Number(1.into())), "1");
        assert_eq!(format!("{}", Value::String("foo".into())), "\"foo\"");
        assert_eq!(format!("{}", Value::Error(Error::Type)), "#TYPE");
    }

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&Value::Empty).unwrap(),
            r#"{"type":"Empty"}"#
        );
        assert_eq!(
            serde_json::to_string(&Value::Number(1.into())).unwrap(),
            r#"{"type":"Number","value":"1"}"#
        );
        assert_eq!(
            serde_json::to_string(&Value::String("foo".into())).unwrap(),
            r#"{"type":"String","value":"foo"}"#
        );
        assert_eq!(
            serde_json::to_string(&Value::Error(Error::Type)).unwrap(),
            r#"{"type":"Error","value":"Type"}"#
        );
    }
}

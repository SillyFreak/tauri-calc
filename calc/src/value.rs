mod formula;

use std::fmt;

use bigdecimal::BigDecimal;

pub use formula::Formula;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Empty,
    Number(BigDecimal),
    String(String),
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
mod test {
    use super::*;

    #[test]
    fn test_value() {
        assert_eq!(format!("{}", Value::Empty), "");
        assert_eq!(format!("{}", Value::Number("1".parse().unwrap())), "1");
        assert_eq!(format!("{}", Value::String("foo".to_string())), "\"foo\"");
    }
}

use std::fmt;

use crate::address::CellAddress;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Formula {
    Reference(CellAddress),
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Reference(value) => write!(f, "{}", value),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Integer(i64),
    Decimal(f64),
    String(String),
    Formula(Formula),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer(value) => write!(f, "{:?}", value),
            Self::Decimal(value) => write!(f, "{:?}", value),
            Self::String(value) => write!(f, "{:?}", value),
            Self::Formula(value) => write!(f, "={}", value),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_value() {
        assert_eq!(format!("{}", Value::Integer(1)), "1");
        assert_eq!(format!("{}", Value::Decimal(1.0)), "1.0");
        assert_eq!(format!("{}", Value::String("foo".to_string())), "\"foo\"");
        let cell_address = CellAddress::new(1.try_into().unwrap(), 1.try_into().unwrap());
        let formula = Value::Formula(Formula::Reference(cell_address));
        assert_eq!(format!("{}", formula), "=A1");
    }
}

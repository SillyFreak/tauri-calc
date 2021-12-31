use crate::address::CellAddress;
use std::fmt;

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

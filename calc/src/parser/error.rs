use std::num::{IntErrorKind, ParseIntError};

use thiserror::Error;

/// An error while parsind a column address such as "C" or "AA".
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseColumnAddressError {
    #[error("parsed string was empty")]
    Empty,
    #[error("parsed string contained invalid characters")]
    InvalidCharacter,
    #[error("parsed string represents a too large address")]
    Overflow,
}

/// An error while parsind a row address such as "1" or "11".
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseRowAddressError {
    #[error("parsed string was empty")]
    Empty,
    #[error("parsed string contained invalid characters")]
    InvalidCharacter,
    #[error("parsed string represents a too large address")]
    Overflow,
    #[error("parsed string represents a too large address")]
    Zero,
    #[error("unknown integer parsing error")]
    Other(ParseIntError),
}

impl From<ParseIntError> for ParseRowAddressError {
    fn from(err: ParseIntError) -> Self {
        match err.kind() {
            IntErrorKind::Empty => Self::Empty,
            IntErrorKind::InvalidDigit => Self::InvalidCharacter,
            IntErrorKind::PosOverflow => Self::Overflow,
            IntErrorKind::Zero => Self::Zero,

            // in our usual parsing, negative signs are never allowed
            // still, for unusual uses, best preserve that as well as unknown error kinds
            IntErrorKind::NegOverflow => Self::Other(err),
            _ => Self::Other(err),
        }
    }
}

/// An error while parsind a column address such as "C" or "AA".
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseCellAddressError {
    #[error("parsed string was not a valid cell address")]
    Invalid,
}

/// An error while parsind a column address such as "C" or "AA".
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseFormulaError {
    #[error("parsed string was not a valid formula")]
    Invalid,
}

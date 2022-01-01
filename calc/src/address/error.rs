use thiserror::Error;

/// An error while parsind a column address such as "C" or "AA".
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseColumnError {
    #[error("parsed string was empty")]
    Empty,
    #[error("parsed string contained invalid characters")]
    InvalidCharacter,
    #[error("parsed string represents a too large address")]
    Overflow,
}

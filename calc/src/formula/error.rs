use thiserror::Error;

/// An error while parsind a column address such as "C" or "AA".
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FormulaError {
    #[error("parsed string was not a valid formula")]
    Invalid,
}

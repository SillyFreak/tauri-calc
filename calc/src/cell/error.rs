use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellInputError {
    pub(super) kind: CellInputErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CellInputErrorKind {
    InvalidFormula,
}

impl CellInputError {
    pub fn kind(&self) -> &CellInputErrorKind {
        &self.kind
    }
}

impl fmt::Display for CellInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.kind {
                CellInputErrorKind::InvalidFormula => "cannot parse formula",
            }
        )
    }
}

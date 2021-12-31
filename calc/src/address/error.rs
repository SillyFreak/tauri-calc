use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseColumnError {
    pub(super) kind: ColumnErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ColumnErrorKind {
    Empty,
    InvalidCharacter,
    Overflow,
}

impl ParseColumnError {
    pub fn kind(&self) -> &ColumnErrorKind {
        &self.kind
    }
}

impl fmt::Display for ParseColumnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.kind {
                ColumnErrorKind::Empty => "cannot parse column from empty string",
                ColumnErrorKind::InvalidCharacter => "invalid character found in string",
                ColumnErrorKind::Overflow => "column too large to fit in target type",
            }
        )
    }
}

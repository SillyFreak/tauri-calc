use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormulaError {
    pub(super) kind: FormulaErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FormulaErrorKind {
    InvalidFormula,
}

impl FormulaError {
    pub fn kind(&self) -> &FormulaErrorKind {
        &self.kind
    }
}

impl fmt::Display for FormulaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.kind {
                FormulaErrorKind::InvalidFormula => "cannot parse formula",
            }
        )
    }
}

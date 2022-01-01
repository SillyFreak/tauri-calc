use crate::formula::Formula;
use crate::value::Value;

#[derive(Clone, Debug, Default)]
pub struct Cell {
    pub(crate) input: String,
    pub(crate) formula: Formula,
    pub(crate) value: Value,
}

impl Cell {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn formula(&self) -> &Formula {
        &self.formula
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}

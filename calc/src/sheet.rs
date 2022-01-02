use std::collections::HashMap;

use crate::address::CellAddress;
use crate::cell::Cell;
use crate::formula::{Evaluate, FormulaError};
use crate::value::{Error, Value};

#[derive(Default)]
pub struct Sheet {
    cells: HashMap<CellAddress, Cell>,
    functions: HashMap<String, Box<dyn Fn(&[Value]) -> Value>>,
}

impl Sheet {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn cell(&self, address: &CellAddress) -> Option<&Cell> {
        self.cells.get(address)
    }

    pub fn value(&self, address: &CellAddress) -> Option<&Value> {
        self.cell(address).map(Cell::value)
    }

    pub fn evaluate<T: Evaluate>(&self, expression: &T) -> Value {
        expression.evaluate(self)
    }

    pub fn set_cell(&mut self, address: CellAddress, input: String) -> Result<(), FormulaError> {
        let formula = input.parse()?;
        let value = self.evaluate(&formula);

        let mut cell = self.cells.entry(address).or_default();
        cell.input = input;
        cell.formula = formula;
        cell.value = value;

        Ok(())
    }

    pub fn reevaluate(&mut self, address: &CellAddress) {
        let cell = match self.cell(address) {
            Some(cell) => cell,
            None => return,
        };

        let value = self.evaluate(&cell.formula);

        let mut cell = self.cells.get_mut(address).expect("cell doesn't exist");
        cell.value = value;
    }

    pub fn function(&self, name: &str) -> Option<&dyn Fn(&[Value]) -> Value> {
        self.functions.get(name).map(Box::as_ref)
    }

    pub fn call(&self, name: &str, arguments: &[Value]) -> Value {
        match self.function(name) {
            Some(function) => function(arguments),
            None => return Value::Error(Error::Undefined),
        }
    }

    pub fn set_function<S: ToString, F: 'static + Fn(&[Value]) -> Value>(
        &mut self,
        name: S,
        function: F,
    ) {
        self.functions.insert(name.to_string(), Box::new(function));
    }
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;

    use super::*;

    use crate::value::Value;

    fn set_and_get_cell<S: ToString>(input: S) -> Value {
        let mut sheet = Sheet::new();
        sheet
            .set_cell("A1".parse().unwrap(), input.to_string())
            .unwrap();
        sheet.value(&"A1".parse().unwrap()).into()
    }

    #[test]
    fn test_empty() {
        let value = set_and_get_cell("");
        assert_eq!(value, Value::Empty);
    }

    #[test]
    fn test_literal_number() {
        let value = set_and_get_cell("1");
        assert_eq!(value, Value::Number(1.into()));
    }

    #[test]
    fn test_literal_string() {
        let value = set_and_get_cell("\"foo\"");
        assert_eq!(value, Value::String("foo".into()));
    }

    #[test]
    fn test_number_formula() {
        let value = set_and_get_cell("=1");
        assert_eq!(value, Value::Number(1.into()));
    }

    #[test]
    fn test_string_formula() {
        let value = set_and_get_cell("=\"foo\"");
        assert_eq!(value, Value::String("foo".into()));
    }

    #[test]
    fn test_reference() {
        let mut sheet = Sheet::new();
        sheet
            .set_cell("A1".parse().unwrap(), "1".to_string())
            .unwrap();
        sheet
            .set_cell("A2".parse().unwrap(), "=A1".to_string())
            .unwrap();

        let value: Value = sheet.value(&"A2".parse().unwrap()).into();
        assert_eq!(value, Value::Number(1.into()));
    }

    #[test]
    fn test_reference_reevaluate() {
        let mut sheet = Sheet::new();
        sheet
            .set_cell("A2".parse().unwrap(), "=A1".to_string())
            .unwrap();
        sheet
            .set_cell("A1".parse().unwrap(), "1".to_string())
            .unwrap();

        let value: Value = sheet.value(&"A2".parse().unwrap()).into();
        assert_eq!(value, Value::Empty);

        sheet.reevaluate(&"A2".parse().unwrap());

        let value: Value = sheet.value(&"A2".parse().unwrap()).into();
        assert_eq!(value, Value::Number(1.into()));
    }

    #[test]
    fn test_function() {
        let mut sheet = Sheet::new();
        sheet.set_function("fn", |arguments| match arguments {
            [arg] => arg.clone(),
            _ => Value::Error(Error::Type),
        });

        sheet
            .set_cell("A1".parse().unwrap(), "=fn(1)".to_string())
            .unwrap();

        let value: Value = sheet.value(&"A1".parse().unwrap()).into();
        assert_eq!(value, Value::Number(1.into()));
    }

    #[test]
    fn test_sum_function() {
        fn sum(arguments: &[Value]) -> Value {
            fn sum(args: &[Value]) -> Result<BigDecimal, Error> {
                let mut sum = 0.into();

                for arg in args {
                    let arg = arg.as_number()?;
                    sum += arg;
                }

                Ok(sum)
            }

            match sum(arguments) {
                Ok(value) => Value::Number(value),
                Err(error) => Value::Error(error),
            }
        }

        let mut sheet = Sheet::new();
        sheet.set_function("sum", sum);

        sheet
            .set_cell("A1".parse().unwrap(), "=sum(1,2,3)".to_string())
            .unwrap();

        let value: Value = sheet.value(&"A1".parse().unwrap()).into();
        assert_eq!(value, Value::Number(6.into()));
    }
}

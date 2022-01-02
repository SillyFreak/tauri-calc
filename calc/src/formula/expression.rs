use crate::address::CellAddress;
use crate::sheet::Sheet;
use crate::value::Value;

use super::Evaluate;

#[derive(Clone, Debug)]
pub enum Expression {
    Literal(Value),
    Reference(CellAddress),
    Call {
        name: String,
        arguments: Vec<Expression>,
    },
}

impl Evaluate for Expression {
    fn evaluate(&self, context: &Sheet) -> Value {
        match self {
            Self::Literal(value) => value.clone(),
            Self::Reference(address) => context.value(address).into(),
            Self::Call { name, arguments } => {
                let arguments: Vec<_> = arguments.iter().map(|arg| arg.evaluate(context)).collect();
                context.call(name, &arguments)
            }
        }
    }
}

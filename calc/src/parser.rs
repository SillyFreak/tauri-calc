//! Parsers for different elements that can be put into a cell

mod formula;
mod identifier;
mod number;
mod range;
mod string;

use nom::branch::alt;
use nom::combinator::{eof, map};
use nom::IResult;

use crate::formula::Formula;
use crate::value::Value;

pub use self::range::cell_address;

pub fn parse_cell(input: &str) -> IResult<&str, Formula> {
    alt((
        map(formula::parse_formula, Formula::Formula),
        map(parse_value, Formula::Literal),
    ))(input)
}

/// parses a cell value that is entered plainly into a cell
pub fn parse_value(input: &str) -> IResult<&str, Value> {
    alt((
        map(eof, |_| Value::Empty),
        map(number::parse_number, Value::Number),
        // TODO plain entered strings with quotes & escaping or verbatim?
        map(string::parse_string, Value::String),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::value::Value;

    #[test]
    fn test_parse() {
        assert_eq!(parse_value("").unwrap().1, Value::Empty);
        assert_eq!(
            parse_value("1").unwrap().1,
            Value::Number("1".parse().unwrap())
        );
        assert_eq!(
            parse_value("\"foo\"").unwrap().1,
            Value::String("foo".to_string())
        );
        assert!(matches!(parse_value("x"), Err(_)));
    }
}

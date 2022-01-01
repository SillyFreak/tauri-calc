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
    let formula = map(formula::parse_formula, |f| Formula::Formula(f));
    let value = map(parse_value, |v| Formula::Literal(v));

    alt((formula, value))(input)
}

/// parses a cell value that is entered plainly into a cell
pub fn parse_value(input: &str) -> IResult<&str, Value> {
    let empty = map(eof, |_| Value::Empty);
    let number = map(number::parse_number, |n| Value::Number(n));
    // TODO plain entered strings with quotes & escaping or verbatim?
    let string = map(string::parse_string, |s| Value::String(s));

    alt((empty, number, string))(input)
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

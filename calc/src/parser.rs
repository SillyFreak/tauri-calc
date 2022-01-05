//! Parsers for different elements that can be put into a cell

mod error;
mod formula;
mod identifier;
mod number;
pub mod range;
mod string;

use nom::branch::alt;
use nom::character::complete::space0;
use nom::combinator::{all_consuming, eof, map};
use nom::error::ParseError;
use nom::sequence::delimited;
use nom::{AsChar, Err, InputLength, InputTakeAtPosition};
use nom::{IResult, Parser};

use crate::formula::Formula;
use crate::value::Value;

pub use self::error::*;

fn parse_complete<I, O, E: ParseError<I>, F>(f: F, input: I) -> Result<O, Err<E>>
where
    I: InputLength + InputTakeAtPosition,
    F: Parser<I, O, E>,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    let (_, res) = all_consuming(delimited(space0, f, space0))(input)?;
    Ok(res)
}

pub fn parse_cell_complete(input: &str) -> Result<Formula, ParseFormulaError> {
    parse_complete(parse_cell, input).map_err(|_| ParseFormulaError::Invalid)
}

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
    fn test_parse_value() {
        let parse_value = |s| parse_complete(parse_value, s);

        assert_eq!(parse_value("  ").unwrap(), Value::Empty);
        assert_eq!(
            parse_value(" 1 ").unwrap(),
            Value::Number("1".parse().unwrap())
        );
        assert_eq!(
            parse_value(" \"foo\" ").unwrap(),
            Value::String("foo".to_string())
        );
        assert!(matches!(parse_value("x"), Err(_)));
    }
}

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;

use crate::formula::expression::Expression;
use crate::value::Value;

use super::number::parse_number;
use super::range::parse_range;
use super::string::parse_string;

pub fn parse_formula(input: &str) -> IResult<&str, Expression> {
    let formula = alt((
        map(parse_range, |address| Expression::Reference(address)),
        map(parse_number, |number| {
            Expression::Literal(Value::Number(number))
        }),
        map(parse_string, |string| {
            Expression::Literal(Value::String(string))
        }),
    ));

    preceded(tag("="), formula)(input)
}

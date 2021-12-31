use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;

use super::range::parse_range;
use crate::value::Formula;

pub fn parse_formula(input: &str) -> IResult<&str, Formula> {
    let formula = alt((
        map(parse_range, |address| Formula::Reference(address)),
    ));

    preceded(tag("="), formula)(input)
}

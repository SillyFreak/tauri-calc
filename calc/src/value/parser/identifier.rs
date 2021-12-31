use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1};
use nom::combinator::recognize;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;

pub fn parse_identifier(input: &str) -> IResult<&str, &str> {
    let identifier_start = alt((alpha1, tag("_")));
    let identifier_continue = alt((alphanumeric1, tag("_")));

    recognize(tuple((identifier_start, many0(identifier_continue))))(input)
}

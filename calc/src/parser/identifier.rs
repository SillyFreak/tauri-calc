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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::parser::parse_complete;

    #[test]
    fn test_parse_identifier() {
        let parse_identifier = |s| parse_complete(parse_identifier, s);

        assert!(parse_identifier("_").is_ok());
        assert!(parse_identifier("foo").is_ok());
        assert!(parse_identifier("_foo").is_ok());
        assert!(parse_identifier("f00").is_ok());
        assert!(parse_identifier("_f00").is_ok());
        assert!(parse_identifier("_00f").is_ok());
        assert!(parse_identifier("").is_err());
        assert!(parse_identifier("00f").is_err());
        assert!(parse_identifier("").is_err());
    }
}

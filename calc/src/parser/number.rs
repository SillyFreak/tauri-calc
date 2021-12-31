//! Parsing of various kinds of numbers (bin, oct, hex, dec, decimal floating point numbers)
//! into [`BigDecimal`].

use bigdecimal::BigDecimal;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit0, digit1, hex_digit1, oct_digit1, one_of};
use nom::combinator::{map_res, opt, recognize};
use nom::error::FromExternalError;
use nom::multi::many1;
use nom::sequence::{preceded, tuple};
use nom::IResult;
use nom::Parser;

pub fn parse_number(input: &str) -> IResult<&str, BigDecimal> {
    alt((binary, octal, decimal, hexadecimal))(input)
}

fn number<'a, F, E>(parser: F) -> impl FnMut(&'a str) -> IResult<&'a str, BigDecimal, E>
where
    E: FromExternalError<&'a str, bigdecimal::ParseBigDecimalError>,
    F: Parser<&'a str, &'a str, E>,
{
    map_res(parser, |out: &str| out.parse())
}

fn binary(input: &str) -> IResult<&str, BigDecimal> {
    let bin_digit1 = recognize(many1(one_of("01")));

    // TODO sign
    number(preceded(alt((tag("0b"), tag("0b"))), bin_digit1))(input)
}

fn octal(input: &str) -> IResult<&str, BigDecimal> {
    // TODO sign
    number(preceded(alt((tag("0o"), tag("0O"))), oct_digit1))(input)
}

fn decimal(input: &str) -> IResult<&str, BigDecimal> {
    let decimal_str = alt((
        // .42[e42] (decimal part required)
        recognize(tuple((
            opt(one_of("+-")),
            char('.'),
            digit1,
            opt(tuple((one_of("eE"), opt(one_of("+-")), digit1))),
        ))),
        // 42[.[42]][e42] (integral part required)
        recognize(tuple((
            opt(one_of("+-")),
            digit1,
            opt(tuple((char('.'), opt(digit0)))),
            opt(tuple((one_of("eE"), opt(one_of("+-")), digit1))),
        ))),
    ));

    number(decimal_str)(input)
}

fn hexadecimal(input: &str) -> IResult<&str, BigDecimal> {
    // TODO sign
    number(preceded(alt((tag("0x"), tag("0X"))), hex_digit1))(input)
}

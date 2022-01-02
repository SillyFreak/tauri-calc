//! Parsing of various kinds of numbers (bin, oct, hex, dec, decimal floating point numbers)
//! into [`BigDecimal`].

use bigdecimal::{BigDecimal, Num};
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

fn signed_prefixed_number<'a, F, G, E>(
    prefix: F,
    digits: G,
    radix: u32,
) -> impl FnMut(&'a str) -> IResult<&'a str, BigDecimal, E>
where
    E: FromExternalError<&'a str, bigdecimal::ParseBigDecimalError>
        + nom::error::ParseError<&'a str>,
    F: Parser<&'a str, &'a str, E>,
    G: Parser<&'a str, &'a str, E>,
{
    let sign = opt(one_of("+-"));
    map_res(
        tuple((sign, preceded(prefix, digits))),
        move |(sign, str)| {
            let number = BigDecimal::from_str_radix(str, radix)?;
            let number = match sign {
                Some('+') | None => number,
                Some('-') => -number,
                Some(_) => unreachable!("sign could only be +, -, or missing"),
            };
            Ok(number)
        },
    )
}

fn binary(input: &str) -> IResult<&str, BigDecimal> {
    let prefix = alt((tag("0b"), tag("0b")));
    let bin_digit1 = recognize(many1(one_of("01")));

    signed_prefixed_number(prefix, bin_digit1, 2)(input)
}

fn octal(input: &str) -> IResult<&str, BigDecimal> {
    let prefix = alt((tag("0o"), tag("0O")));

    signed_prefixed_number(prefix, oct_digit1, 8)(input)
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

    map_res(decimal_str, |out: &str| out.parse())(input)
}

fn hexadecimal(input: &str) -> IResult<&str, BigDecimal> {
    let prefix = alt((tag("0x"), tag("0X")));

    signed_prefixed_number(prefix, hex_digit1, 16)(input)
}

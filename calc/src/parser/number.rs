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
use num_bigint::BigInt;

pub fn parse_number(input: &str) -> IResult<&str, BigDecimal> {
    alt((binary, octal, decimal, hexadecimal))(input)
}

fn signed_prefixed_int<'a, F, G, E>(
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
            let number = BigInt::from_str_radix(str, radix)?;
            let number = BigDecimal::from(number);
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
    let prefix = alt((tag("0b"), tag("0B")));
    let bin_digit1 = recognize(many1(one_of("01")));

    signed_prefixed_int(prefix, bin_digit1, 2)(input)
}

fn octal(input: &str) -> IResult<&str, BigDecimal> {
    let prefix = alt((tag("0o"), tag("0O")));

    signed_prefixed_int(prefix, oct_digit1, 8)(input)
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

    signed_prefixed_int(prefix, hex_digit1, 16)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::parser::parse_complete;

    #[test]
    fn test_parse_binary() {
        let binary = |s| parse_complete(binary, s);

        assert_eq!(binary("0b0").unwrap(), 0.into());
        assert_eq!(binary("0b10").unwrap(), 2.into());
        assert_eq!(binary("-0b1").unwrap(), (-1).into());
        assert_eq!(binary("-0b10").unwrap(), (-2).into());
        assert_eq!(binary("+0b1").unwrap(), 1.into());
        assert_eq!(binary("+0b10").unwrap(), 2.into());
        assert_eq!(binary("0B0").unwrap(), 0.into());
        assert_eq!(binary("0B10").unwrap(), 2.into());
        assert_eq!(binary("-0B1").unwrap(), (-1).into());
        assert_eq!(binary("-0B10").unwrap(), (-2).into());
        assert_eq!(binary("+0B1").unwrap(), 1.into());
        assert_eq!(binary("+0B10").unwrap(), 2.into());
        assert!(binary("").is_err());
        assert!(binary("010").is_err());
        assert!(binary("asd").is_err());
        assert!(binary("0x0").is_err());
    }

    #[test]
    fn test_parse_octal() {
        let octal = |s| parse_complete(octal, s);

        assert_eq!(octal("0o0").unwrap(), 0.into());
        assert_eq!(octal("0o10").unwrap(), 8.into());
        assert_eq!(octal("-0o7").unwrap(), (-7).into());
        assert_eq!(octal("-0o10").unwrap(), (-8).into());
        assert_eq!(octal("+0o7").unwrap(), 7.into());
        assert_eq!(octal("+0o10").unwrap(), 8.into());
        assert_eq!(octal("0O0").unwrap(), 0.into());
        assert_eq!(octal("0O10").unwrap(), 8.into());
        assert_eq!(octal("-0O7").unwrap(), (-7).into());
        assert_eq!(octal("-0O10").unwrap(), (-8).into());
        assert_eq!(octal("+0O7").unwrap(), 7.into());
        assert_eq!(octal("+0O10").unwrap(), 8.into());
        assert!(octal("").is_err());
        assert!(octal("010").is_err());
        assert!(octal("asd").is_err());
        assert!(octal("0x0").is_err());
    }

    #[test]
    fn test_parse_hexadecimal() {
        let hexadecimal = |s| parse_complete(hexadecimal, s);

        assert_eq!(hexadecimal("0x0").unwrap(), 0.into());
        assert_eq!(hexadecimal("0x10").unwrap(), 16.into());
        assert_eq!(hexadecimal("-0xa").unwrap(), (-10).into());
        assert_eq!(hexadecimal("-0x10").unwrap(), (-16).into());
        assert_eq!(hexadecimal("+0xA").unwrap(), 10.into());
        assert_eq!(hexadecimal("+0x10").unwrap(), 16.into());
        assert_eq!(hexadecimal("0X0").unwrap(), 0.into());
        assert_eq!(hexadecimal("0X10").unwrap(), 16.into());
        assert_eq!(hexadecimal("-0XA").unwrap(), (-10).into());
        assert_eq!(hexadecimal("-0X10").unwrap(), (-16).into());
        assert_eq!(hexadecimal("+0Xa").unwrap(), 10.into());
        assert_eq!(hexadecimal("+0X10").unwrap(), 16.into());
        assert!(hexadecimal("").is_err());
        assert!(hexadecimal("010").is_err());
        assert!(hexadecimal("asd").is_err());
        assert!(hexadecimal("0b0").is_err());
    }
}

mod formula;
mod identifier;
mod number;
mod range;
mod string;

use nom::branch::alt;
use nom::combinator::{eof, map};
use nom::IResult;

use crate::value::Value;

pub fn parse_value(input: &str) -> IResult<&str, Value> {
    let empty = map(eof, |_| Value::Empty);
    let number = map(number::parse_number, |n| Value::Number(n));
    let string = map(string::parse_string, |s| Value::String(s));

    alt((empty, number, string))(input)
}

#[cfg(test)]
mod test {
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

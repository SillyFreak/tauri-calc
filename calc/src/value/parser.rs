mod formula;
mod identifier;
mod number;
mod range;
mod string;

use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

use super::Value;

pub fn parse(input: &str) -> IResult<&str, Option<Value>> {
    let formula = map(formula::parse_formula, |f| Some(Value::Formula(f)));
    let number = map(number::parse_number, |n| Some(Value::Number(n)));
    let string = map(string::parse_string, |s| Some(Value::String(s)));

    alt((formula, number, string))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::address::CellAddress;
    use crate::value::{Formula, Value};

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("1").unwrap().1.unwrap(),
            Value::Number("1".parse().unwrap())
        );
        assert_eq!(
            parse("\"foo\"").unwrap().1.unwrap(),
            Value::String("foo".to_string())
        );
        let cell_address = CellAddress::new(1.try_into().unwrap(), 1.try_into().unwrap());
        let formula = Value::Formula(Formula::Reference(cell_address));
        assert_eq!(parse("=A1").unwrap().1.unwrap(), formula);
    }
}

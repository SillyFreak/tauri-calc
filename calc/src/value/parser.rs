use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple,
    IResult,
};

use super::Value;

pub fn parse(expr: &str) -> IResult<&str, Option<Value>> {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::address::CellAddress;
    use crate::value::{Formula, Value};

    #[test]
    fn test_parse() {
        assert_eq!(parse("1").unwrap().1.unwrap(), Value::Number("1".parse().unwrap()));
        assert_eq!(
            parse("\"foo\"").unwrap().1.unwrap(),
            Value::String("foo".to_string())
        );
        let cell_address = CellAddress::new(1.try_into().unwrap(), 1.try_into().unwrap());
        let formula = Value::Formula(Formula::Reference(cell_address));
        assert_eq!(parse("=A1").unwrap().1.unwrap(), formula);
    }
}

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::character::streaming::char;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};
use nom::IResult;

use crate::formula::expression::Expression;
use crate::value::Value;

use super::identifier::parse_identifier;
use super::number::parse_number;
use super::range::parse_range;
use super::string::parse_string;

pub fn parse_formula(input: &str) -> IResult<&str, Expression> {
    preceded(tuple((tag("="), space0)), parse_expression)(input)
}

pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        map(parse_range, Expression::Reference),
        map(parse_literal, Expression::Literal),
        parse_call,
    ))(input)
}

pub fn parse_literal(input: &str) -> IResult<&str, Value> {
    alt((
        map(parse_number, Value::Number),
        map(parse_string, Value::String),
    ))(input)
}

pub fn parse_call(input: &str) -> IResult<&str, Expression> {
    let args = map(
        // (arg ,)* arg?
        tuple((
            many0(terminated(
                parse_expression,
                delimited(space0, char(','), space0),
            )),
            opt(parse_expression),
        )),
        // combine the repeated args with the optional trailing arg
        |(mut args, trailing_arg)| {
            args.extend(trailing_arg);
            args
        },
    );
    let args = delimited(tuple((char('('), space0)), args, tuple((space0, char(')'))));

    map(
        separated_pair(parse_identifier, space0, args),
        |(name, arguments)| {
            let name = name.to_string();
            Expression::Call { name, arguments }
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::parser::parse_complete;
    use crate::value::Value;

    #[test]
    fn test_parse_literal() {
        let parse_literal = |s| parse_complete(parse_literal, s);

        assert_eq!(
            parse_literal(" 1 ").unwrap(),
            Value::Number("1".parse().unwrap())
        );
        assert_eq!(
            parse_literal(" \"foo\" ").unwrap(),
            Value::String("foo".to_string())
        );

        assert!(parse_literal("foo").is_err());
    }

    #[test]
    fn test_parse_call() {
        let parse_call = |s| parse_complete(parse_call, s);

        assert!(matches!(
            parse_call(" foo ( ) ").unwrap(),
            Expression::Call { name, arguments } if name == "foo" && arguments.len() == 0,
        ));
        assert!(matches!(
            parse_call(" foo ( 1 ) ").unwrap(),
            Expression::Call { name, arguments } if name == "foo" && arguments.len() == 1,
        ));
        assert!(matches!(
            parse_call(" foo ( 1 , ) ").unwrap(),
            Expression::Call { name, arguments } if name == "foo" && arguments.len() == 1,
        ));
        assert!(matches!(
            parse_call(" foo ( 1 , 1 ) ").unwrap(),
            Expression::Call { name, arguments } if name == "foo" && arguments.len() == 2,
        ));
        assert!(matches!(
            parse_call(" foo ( 1 , 1 , ) ").unwrap(),
            Expression::Call { name, arguments } if name == "foo" && arguments.len() == 2,
        ));

        assert!(parse_call("foo").is_err());
        assert!(parse_call("foo(").is_err());
        assert!(parse_call("foo(1").is_err());
        assert!(parse_call("foo(,1)").is_err());
    }
}

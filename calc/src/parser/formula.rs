use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::streaming::char;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

use crate::formula::expression::Expression;
use crate::value::Value;

use super::identifier::parse_identifier;
use super::number::parse_number;
use super::range::parse_range;
use super::string::parse_string;

pub fn parse_formula(input: &str) -> IResult<&str, Expression> {
    preceded(tag("="), parse_expression)(input)
}

pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        map(parse_range, |address| Expression::Reference(address)),
        map(parse_number, |number| {
            Expression::Literal(Value::Number(number))
        }),
        map(parse_string, |string| {
            Expression::Literal(Value::String(string))
        }),
        parse_call,
    ))(input)
}

pub fn parse_call(input: &str) -> IResult<&str, Expression> {
    let args = tuple((
        many0(terminated(parse_expression, char(','))),
        opt(parse_expression),
    ));
    let args = delimited(char('('), args, char(')'));
    let args = map(args, |(mut args, extra_arg)| {
        if let Some(arg) = extra_arg {
            args.push(arg);
        }
        args
    });

    map(tuple((parse_identifier, args)), |(name, arguments)| {
        let name = name.to_string();
        Expression::Call { name, arguments }
    })(input)
}

#[cfg(test)]
mod tests {
    use crate::parser::formula::parse_formula;

    #[test]
    fn test_parse_call() {
        println!("{:?}", parse_formula("=foo()"));
        println!("{:?}", parse_formula("=foo(1)"));
        println!("{:?}", parse_formula("=foo(1,)"));
        println!("{:?}", parse_formula("=foo(1,1)"));
        println!("{:?}", parse_formula("=foo(1,1,)"));
        println!("{:?}", parse_formula("=foo(1,1,1)"));
        // panic!();
    }
}

use std::str::FromStr;

use nom::character::complete::{alpha1, digit1};
use nom::combinator::{map, map_res};
use nom::sequence::tuple;
use nom::IResult;

use crate::address::{CellAddress, ColAddress, RowAddress};

pub fn parse_range(input: &str) -> IResult<&str, CellAddress> {
    cell_address(input)
}

fn cell_address(input: &str) -> IResult<&str, CellAddress> {
    map(tuple((col_address, row_address)), |(c, r)| {
        CellAddress::new(r, c)
    })(input)
}

fn col_address(input: &str) -> IResult<&str, ColAddress> {
    map_res(alpha1, ColAddress::from_str)(input)
}

fn row_address(input: &str) -> IResult<&str, RowAddress> {
    map_res(digit1, RowAddress::from_str)(input)
}

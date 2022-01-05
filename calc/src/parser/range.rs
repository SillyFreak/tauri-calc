use nom::character::complete::{alpha1, digit1};
use nom::combinator::{map, map_res};
use nom::sequence::tuple;
use nom::IResult;

use crate::address::{CellAddress, ColAddress, RowAddress};
use crate::parser::ParseRowAddressError;

use super::{parse_complete, ParseCellAddressError, ParseColumnAddressError};

pub fn parse_cell_address_complete(input: &str) -> Result<CellAddress, ParseCellAddressError> {
    parse_complete(cell_address, input).map_err(|_| ParseCellAddressError::Invalid)
}

pub fn parse_col_address_complete(input: &str) -> Result<ColAddress, ParseColumnAddressError> {
    parse_complete(col_address, input).map_err(|_| ParseColumnAddressError::InvalidCharacter)
}

pub fn parse_row_address_complete(input: &str) -> Result<RowAddress, ParseRowAddressError> {
    parse_complete(row_address, input).map_err(|_| ParseRowAddressError::InvalidCharacter)
}

pub fn parse_range(input: &str) -> IResult<&str, CellAddress> {
    cell_address(input)
}

pub fn cell_address(input: &str) -> IResult<&str, CellAddress> {
    map(tuple((col_address, row_address)), |(c, r)| {
        CellAddress::new(r, c)
    })(input)
}

fn col_address(input: &str) -> IResult<&str, ColAddress> {
    use std::num::NonZeroU32;

    fn base26digit(ch: char) -> Result<u32, ParseColumnAddressError> {
        match ch.to_digit(36) {
            Some(digit) if digit >= 10 => Ok(digit - 10),
            _ => Err(ParseColumnAddressError::InvalidCharacter),
        }
    }

    map_res(
        alpha1,
        |s: &str| -> Result<ColAddress, ParseColumnAddressError> {
            if s.is_empty() {
                return Err(ParseColumnAddressError::Empty);
            }

            let mut address: u32 = 0;

            for ch in s.chars() {
                let d = base26digit(ch)?;

                address = address
                    .checked_mul(26)
                    .ok_or(ParseColumnAddressError::Overflow)?
                    .checked_add(d)
                    .ok_or(ParseColumnAddressError::Overflow)?;
            }

            let address = NonZeroU32::new(address + 1).unwrap();
            Ok(ColAddress::new(address))
        },
    )(input)
}

fn row_address(input: &str) -> IResult<&str, RowAddress> {
    use std::num::NonZeroU32;

    map_res(
        digit1,
        |s: &str| -> Result<RowAddress, ParseRowAddressError> {
            let address: NonZeroU32 = s.parse()?;
            Ok(RowAddress::new(address))
        },
    )(input)
}

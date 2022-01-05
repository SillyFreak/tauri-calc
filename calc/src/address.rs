//! Types for row, column and cell addresses.

use std::fmt;
use std::num::NonZeroU32;
use std::str::FromStr;

use crate::parser::range::{
    parse_cell_address_complete, parse_col_address_complete, parse_row_address_complete,
};
use crate::parser::{ParseCellAddressError, ParseColumnAddressError, ParseRowAddressError};

/// A row address, which is a positive integer
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RowAddress(NonZeroU32);

impl RowAddress {
    pub fn new(address: NonZeroU32) -> Self {
        Self(address)
    }
}

impl FromStr for RowAddress {
    type Err = ParseRowAddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_row_address_complete(s)
    }
}

impl From<NonZeroU32> for RowAddress {
    fn from(address: NonZeroU32) -> Self {
        Self::new(address)
    }
}

impl TryFrom<u32> for RowAddress {
    type Error = ();

    fn try_from(address: u32) -> Result<Self, Self::Error> {
        let address = NonZeroU32::new(address).ok_or(())?;
        Ok(Self::new(address))
    }
}

impl fmt::Display for RowAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A column address, which is an alphabetic base-26 number where "A" is 1, "Z" is 26, "AA" is 27, etc.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColAddress(NonZeroU32);

impl ColAddress {
    pub fn new(address: NonZeroU32) -> Self {
        Self(address)
    }
}

impl FromStr for ColAddress {
    type Err = ParseColumnAddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_col_address_complete(s)
    }
}

impl From<NonZeroU32> for ColAddress {
    fn from(address: NonZeroU32) -> Self {
        Self::new(address)
    }
}

impl TryFrom<u32> for ColAddress {
    type Error = ();

    fn try_from(address: u32) -> Result<Self, Self::Error> {
        let address = NonZeroU32::new(address).ok_or(())?;
        Ok(Self::new(address))
    }
}

impl fmt::Display for ColAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // convert the address into a 0-based number so that we can treat 'A' as the digit 0
        let mut num = u32::from(self.0) - 1;
        // we need at least one byte, so allocate up front
        // more than four (always ASCII) characters are only needed for outlandish numbers of columns
        let mut str = String::with_capacity(4);

        fn base26digit(num: u32) -> Option<char> {
            // converting a number in 10..36 with radix 36 will result in a base 26 letter
            let digit = char::from_digit(num + 10, 36)?;
            let digit = digit.to_ascii_uppercase();
            Some(digit)
        }

        if num == 0 {
            let digit = base26digit(0).unwrap();
            str.push(digit);
        } else {
            while num > 0 {
                let digit = base26digit(num % 26).unwrap();
                str.insert(0, digit);
                num /= 26;
            }
        }

        write!(f, "{}", str)
    }
}

/// Address for a single cell, which consists of a row and column address.
/// This is mainly used for identifying cells in internal use;
/// most user-facing uses of cell addresses are actually single-cell ranges.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CellAddress {
    row: RowAddress,
    col: ColAddress,
}

impl CellAddress {
    pub fn new(row: RowAddress, col: ColAddress) -> Self {
        Self { row, col }
    }

    pub fn col(&self) -> ColAddress {
        self.col
    }

    pub fn row(&self) -> RowAddress {
        self.row
    }
}

impl FromStr for CellAddress {
    type Err = ParseCellAddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_cell_address_complete(s)
    }
}

impl fmt::Display for CellAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.col, self.row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let cell_address = CellAddress::new(1.try_into().unwrap(), 1.try_into().unwrap());
        assert_eq!(format!("{}", cell_address), "A1");
    }
}

//! Types for row, column and cell addresses.

use std::fmt;
use std::num::NonZeroU32;
use std::str::FromStr;

use serde::de::Error;
use serde::de::Visitor;
use serde::{Deserialize, Serialize, Serializer};

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

impl Serialize for RowAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("$tauri_calc::row_address", &self.to_string())
    }
}

impl<'de> Deserialize<'de> for RowAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RowAddressVisitor;

        impl<'de> Visitor<'de> for RowAddressVisitor {
            type Value = RowAddress;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a numerical row address as a string is expected")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse().map_err(Error::custom)
            }
        }

        deserializer.deserialize_str(RowAddressVisitor)
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
        let mut num = u32::from(self.0);
        // we need at least one byte, so allocate up front
        // more than four (always ASCII) characters are only needed for outlandish numbers of columns
        let mut str = String::with_capacity(4);

        fn base26digit(num: u32) -> Option<char> {
            // converting a number in 10..36 with radix 36 will result in a base 26 letter
            let digit = char::from_digit(num + 10, 36)?;
            let digit = digit.to_ascii_uppercase();
            Some(digit)
        }

        // the number is always > 0 in the beginning, so we're never returning an empty string
        while num > 0 {
            // we need to subtract 1 basically to account for the fact that none of the letters stands for zero
            // e.g. for 27:
            // - decrement to 26
            // - 26 % 26 results in A for the least significant digit
            // - 26 / 26 results in 1 for the next iteration
            // - decrement to 0
            // - 0 % 26 results in A for the second digit
            // this off-by-one error basically accumulates because of the 1-based nature of base26 numerals
            num -= 1;
            let digit = base26digit(num % 26).unwrap();
            str.insert(0, digit);
            num /= 26;
        }

        write!(f, "{}", str)
    }
}

impl Serialize for ColAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("$tauri_calc::col_address", &self.to_string())
    }
}

impl<'de> Deserialize<'de> for ColAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ColAddressVisitor;

        impl<'de> Visitor<'de> for ColAddressVisitor {
            type Value = ColAddress;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an alphabetic column address as a string is expected")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse().map_err(Error::custom)
            }
        }

        deserializer.deserialize_str(ColAddressVisitor)
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

impl Serialize for CellAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("$tauri_calc::cell_address", &self.to_string())
    }
}

impl<'de> Deserialize<'de> for CellAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CellAddressVisitor;

        impl<'de> Visitor<'de> for CellAddressVisitor {
            type Value = CellAddress;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an alphabetic column address as a string is expected")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse().map_err(Error::custom)
            }
        }

        deserializer.deserialize_str(CellAddressVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_col() {
        fn col_address(col: u32) -> ColAddress {
            ColAddress::new(col.try_into().unwrap())
        }

        assert_eq!(format!("{}", col_address(1)), "A");
        assert_eq!(format!("{}", col_address(26)), "Z");
        assert_eq!(format!("{}", col_address(1 * 26 + 1)), "AA");
        assert_eq!(format!("{}", col_address(1 * 26 + 26)), "AZ");
        assert_eq!(format!("{}", col_address(26 * 26 + 26)), "ZZ");
        assert_eq!(format!("{}", col_address(1 * 26 * 26 + 1 * 26 + 1)), "AAA");
    }
}

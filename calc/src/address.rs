use std::fmt;
use std::num::NonZeroU32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RowAddress(NonZeroU32);

impl RowAddress {
    pub fn new(address: NonZeroU32) -> Self {
        Self(address)
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColAddress(NonZeroU32);

impl ColAddress {
    pub fn new(address: NonZeroU32) -> Self {
        Self(address)
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

impl From<NonZeroU32> for ColAddress {
    fn from(address: NonZeroU32) -> Self {
        Self::new(address)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellAddress {
    row: RowAddress,
    col: ColAddress,
}

impl CellAddress {
    pub fn new(row: RowAddress, col: ColAddress) -> Self {
        Self { row, col }
    }
}

impl fmt::Display for CellAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.col, self.row)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        let cell_address = CellAddress::new(1.try_into().unwrap(), 1.try_into().unwrap());
        assert_eq!(format!("{}", cell_address), "A1");
    }
}

use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum MaskParsingError {
    InvalidRange,
    InvalidFormat,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Mask(u32);

impl fmt::Display for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b, c, d] = Mask::group_octets(self.0);
        write!(f, "{}.{}.{}.{}", a, b, c, d)
    }
}

impl fmt::Binary for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b, c, d] = Mask::group_octets(self.0);
        write!(f, "{:08b}.{:08b}.{:08b}.{:08b}", a, b, c, d)
    }
}

impl std::str::FromStr for Mask {
    type Err = MaskParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mask: u8 = s.parse().map_err(|_| MaskParsingError::InvalidFormat)?;

        Self::new(mask)
    }
}

impl Mask {
    pub fn new(value: u8) -> Result<Self, MaskParsingError> {
        if value == 0 || value > 32 {
            return Err(MaskParsingError::InvalidRange);
        }

        let mask: u32 = 0xFFFFFFFF << (32 - value);

        Ok(Self(mask))
    }

    pub fn len(&self) -> u8 {
        let mut zeroes = 0;
        for i in 0..32 {
            if self.0 >> i & 1 == 0 {
                zeroes += 1;
                continue;
            }

            break;
        }

        32 - zeroes
    }

    pub fn wildcard(&self) -> Mask {
        let wildcard = self.0 ^ 0xFFFFFFFF;
        Mask(wildcard)
    }

    fn group_octets(value: u32) -> [u8; 4] {
        let a = (value >> 24 & 0xFF) as u8;
        let b = (value >> 16 & 0xFF) as u8;
        let c = (value >> 8 & 0xFF) as u8;
        let d = (value & 0xFF) as u8;

        [a, b, c, d]
    }
}

#[cfg(test)]
mod tests {
    use super::{Mask, MaskParsingError};

    #[test]
    fn parse_mask_negative() {
        assert_eq!(Err(MaskParsingError::InvalidFormat), "-5".parse::<Mask>())
    }

    #[test]
    fn parse_mask_not_number() {
        assert_eq!(Err(MaskParsingError::InvalidFormat), "plop".parse::<Mask>())
    }

    #[test]
    fn parse_mask_not_in_range() {
        assert_eq!(Err(MaskParsingError::InvalidRange), "198".parse::<Mask>())
    }

    #[test]
    fn parse_mask_success() {
        assert_eq!(Mask(0xFFFFFF00), "24".parse::<Mask>().unwrap())
    }

    #[test]
    fn string_display() {
        assert_eq!("255.255.255.0", format!("{}", Mask::new(24).unwrap()));
        assert_eq!("255.255.0.0", format!("{}", Mask::new(16).unwrap()));
        assert_eq!("255.0.0.0", format!("{}", Mask::new(8).unwrap()));
        assert_eq!("255.255.240.0", format!("{}", Mask::new(20).unwrap()));
    }

    #[test]
    fn binary_display() {
        assert_eq!(
            "11111111.11111111.11111111.00000000",
            format!("{:b}", Mask::new(24).unwrap())
        );

        assert_eq!(
            "11111111.11111111.00000000.00000000",
            format!("{:b}", Mask::new(16).unwrap())
        );

        assert_eq!(
            "11111111.11111111.11110000.00000000",
            format!("{:b}", Mask::new(20).unwrap())
        );
    }

    #[test]
    fn get_mask_len() {
        assert_eq!(24, Mask::new(24).unwrap().len())
    }

    #[test]
    fn get_wildcard() {
        assert_eq!(Mask(0x000000FF), Mask::new(24).unwrap().wildcard())
    }
}

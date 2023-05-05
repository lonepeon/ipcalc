use crate::net::IPv4;
use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum MaskParsingError {
    InvalidRange,
    InvalidFormat,
}

#[derive(PartialEq, Eq, Copy, Clone, PartialOrd)]
pub struct Mask(u32);

impl fmt::Debug for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/{}", self.prefix_length())
    }
}

impl fmt::Display for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b, c, d] = crate::net::group_octets(self.0);
        write!(f, "{}.{}.{}.{}", a, b, c, d)
    }
}

impl fmt::Binary for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b, c, d] = crate::net::group_octets(self.0);
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
        if value > 32 {
            return Err(MaskParsingError::InvalidRange);
        }

        let mut mask: u32 = 0;

        if value > 0 {
            mask = 0xFFFFFFFF << (32 - value);
        }

        Ok(Self(mask))
    }

    pub fn prefix_length(&self) -> u8 {
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
        Mask(!self.0)
    }

    pub fn network_address(&self, ip: &IPv4) -> IPv4 {
        let network_address = ip.octets() & self.0;
        IPv4::new_from_raw_bytes(network_address)
    }

    pub fn first_address(&self, ip: &IPv4) -> Option<IPv4> {
        if self.prefix_length() >= 31 {
            return None;
        }

        let address = (ip.octets() & self.0) + 1;
        Some(IPv4::new_from_raw_bytes(address))
    }

    pub fn last_address(&self, ip: &IPv4) -> Option<IPv4> {
        if self.prefix_length() >= 31 {
            return None;
        }

        let address = (ip.octets() & self.0) + (self.wildcard().0 - 1);
        Some(IPv4::new_from_raw_bytes(address))
    }

    pub fn broadcast_address(&self, ip: &IPv4) -> Option<IPv4> {
        if self.prefix_length() >= 31 {
            return None;
        }

        let address = (ip.octets() & self.0) + self.wildcard().0;
        Some(IPv4::new_from_raw_bytes(address))
    }

    pub fn hosts(&self) -> u32 {
        if self.prefix_length() == 32 {
            return 1;
        }
        self.wildcard().0 - 1
    }
}

#[cfg(test)]
mod tests {
    use super::{Mask, MaskParsingError};
    use crate::net::IPv4;

    #[test]
    fn debug_display() {
        assert_eq!("/24", format!("{:?}", Mask::new(24).unwrap()))
    }

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
    fn prefix_length() {
        assert_eq!(24, Mask::new(24).unwrap().prefix_length())
    }

    #[test]
    fn get_wildcard() {
        assert_eq!(Mask(0x00000000), Mask::new(32).unwrap().wildcard());
        assert_eq!(Mask(0x000000FF), Mask::new(24).unwrap().wildcard());
        assert_eq!(Mask(0x0000FFFF), Mask::new(16).unwrap().wildcard());
        assert_eq!(Mask(0x00FFFFFF), Mask::new(8).unwrap().wildcard());
        assert_eq!(Mask(0xFFFFFFFF), Mask::new(0).unwrap().wildcard());
    }

    #[test]
    fn network_address() {
        let host_address = IPv4::new(10, 42, 180, 53);

        let mask = Mask::new(32).unwrap();
        assert_eq!(
            IPv4::new(10, 42, 180, 53),
            mask.network_address(&host_address)
        );

        let mask = Mask::new(24).unwrap();
        assert_eq!(
            IPv4::new(10, 42, 180, 0),
            mask.network_address(&host_address)
        );

        let mask = Mask::new(20).unwrap();
        assert_eq!(
            IPv4::new(10, 42, 176, 0),
            mask.network_address(&host_address)
        );

        let mask = Mask::new(16).unwrap();
        assert_eq!(IPv4::new(10, 42, 0, 0), mask.network_address(&host_address));

        let mask = Mask::new(0).unwrap();
        assert_eq!(IPv4::new(0, 0, 0, 0), mask.network_address(&host_address));
    }

    #[test]
    fn first_address() {
        let host_address = IPv4::new(10, 42, 180, 53);

        let mask = Mask::new(32).unwrap();
        assert_eq!(None, mask.first_address(&host_address));

        let mask = Mask::new(24).unwrap();
        assert_eq!(
            Some(IPv4::new(10, 42, 180, 1)),
            mask.first_address(&host_address)
        );

        let mask = Mask::new(20).unwrap();
        assert_eq!(
            Some(IPv4::new(10, 42, 176, 1)),
            mask.first_address(&host_address)
        );
    }

    #[test]
    fn last_address() {
        let host_address = IPv4::new(10, 42, 180, 53);

        let mask = Mask::new(32).unwrap();
        assert_eq!(None, mask.last_address(&host_address));

        let mask = Mask::new(24).unwrap();
        assert_eq!(
            Some(IPv4::new(10, 42, 180, 254)),
            mask.last_address(&host_address)
        );

        let mask = Mask::new(20).unwrap();
        assert_eq!(
            Some(IPv4::new(10, 42, 191, 254)),
            mask.last_address(&host_address)
        );
    }

    #[test]
    fn broadcast_address() {
        let host_address = IPv4::new(10, 42, 180, 53);

        let mask = Mask::new(32).unwrap();
        assert_eq!(None, mask.broadcast_address(&host_address));

        let mask = Mask::new(24).unwrap();
        assert_eq!(
            Some(IPv4::new(10, 42, 180, 255)),
            mask.broadcast_address(&host_address)
        );

        let mask = Mask::new(20).unwrap();
        assert_eq!(
            Some(IPv4::new(10, 42, 191, 255)),
            mask.broadcast_address(&host_address)
        );
    }

    #[test]
    fn hosts() {
        let mask = Mask::new(32).unwrap();
        assert_eq!(1, mask.hosts());

        let mask = Mask::new(24).unwrap();
        assert_eq!(254, mask.hosts());

        let mask = Mask::new(20).unwrap();
        assert_eq!(4094, mask.hosts());

        let mask = Mask::new(16).unwrap();
        assert_eq!(65534, mask.hosts());

        let mask = Mask::new(8).unwrap();
        assert_eq!(16777214, mask.hosts());
    }
}

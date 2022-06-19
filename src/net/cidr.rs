use crate::net::IPClass;
use crate::net::IPKind;
use crate::net::IPv4;
use crate::net::Mask;
use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum CIDRParsingError {
    InvalidMaskLength,
    InvalidHostFormat,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CIDR {
    pub ip: IPv4,
    pub mask: Mask,
}

impl fmt::Display for CIDR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.ip, self.mask.len())
    }
}

impl std::str::FromStr for CIDR {
    type Err = CIDRParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<&str> = s.split('/').collect();
        if values.len() > 2 {
            return Err(CIDRParsingError::InvalidHostFormat);
        }

        let ip = values[0]
            .parse::<IPv4>()
            .map_err(|_| CIDRParsingError::InvalidHostFormat)?;

        let mask = if values.len() == 2 {
            values[1]
                .parse::<Mask>()
                .map_err(|_| CIDRParsingError::InvalidMaskLength)?
        } else {
            Mask::new(32).unwrap()
        };

        Ok(Self { ip, mask })
    }
}

impl CIDR {
    pub fn network(&self) -> CIDR {
        CIDR {
            ip: self.mask.network_address(&self.ip),
            mask: self.mask,
        }
    }

    pub fn first_address(&self) -> IPv4 {
        self.mask.first_address(&self.ip)
    }

    pub fn last_address(&self) -> IPv4 {
        self.mask.last_address(&self.ip)
    }

    pub fn broadcast_address(&self) -> IPv4 {
        self.mask.broadcast_address(&self.ip)
    }

    pub fn hosts(&self) -> u32 {
        self.mask.hosts()
    }

    pub fn class(&self) -> IPClass {
        self.ip.class()
    }

    pub fn kind(&self) -> IPKind {
        self.ip.kind()
    }
}

#[cfg(test)]
mod tests {
    use super::{CIDRParsingError, CIDR};
    use crate::net;

    #[test]
    fn parse_mask_negative() {
        assert_eq!(
            Err(CIDRParsingError::InvalidMaskLength),
            "10.0.0.0/-5".parse::<CIDR>(),
        )
    }

    #[test]
    fn parse_mask_too_small() {
        assert_eq!(
            Err(CIDRParsingError::InvalidMaskLength),
            "10.0.0.0/38".parse::<CIDR>(),
        )
    }

    #[test]
    fn parse_ip_missing_bytes() {
        assert_eq!(
            Err(CIDRParsingError::InvalidHostFormat),
            "10.0/20".parse::<CIDR>(),
        )
    }

    #[test]
    fn parse_ip_contains_several_slashes() {
        assert_eq!(
            Err(CIDRParsingError::InvalidHostFormat),
            "10.0.10.0/24/12".parse::<CIDR>(),
        )
    }

    #[test]
    fn network() {
        let address = "10.0.10.15/24".parse::<CIDR>().unwrap();
        let expected = "10.0.10.0/24".parse::<CIDR>().unwrap();

        assert_eq!(expected, address.network())
    }

    #[test]
    fn first_address() {
        let address = "10.0.10.15/24".parse::<CIDR>().unwrap();
        let first_address = "10.0.10.1".parse::<net::IPv4>().unwrap();

        assert_eq!(first_address, address.first_address())
    }

    #[test]
    fn last_address() {
        let address = "10.0.10.15/24".parse::<CIDR>().unwrap();
        let last_address = "10.0.10.254".parse::<net::IPv4>().unwrap();

        assert_eq!(last_address, address.last_address())
    }

    #[test]
    fn broadcast_address() {
        let address = "10.0.10.15/24".parse::<CIDR>().unwrap();
        let broadcast_address = "10.0.10.255".parse::<net::IPv4>().unwrap();

        assert_eq!(broadcast_address, address.broadcast_address())
    }

    #[test]
    fn hosts() {
        let address = "10.0.10.15/24".parse::<CIDR>().unwrap();

        assert_eq!(254, address.hosts())
    }

    #[test]
    fn class() {
        let address = "10.0.10.15/24".parse::<CIDR>().unwrap();

        assert_eq!(net::IPClass::A, address.class())
    }

    #[test]
    fn kind() {
        let address = "10.0.10.15/24".parse::<CIDR>().unwrap();

        assert_eq!(net::IPKind::Private, address.kind())
    }
}
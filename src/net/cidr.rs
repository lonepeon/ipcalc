use crate::net::IPClass;
use crate::net::IPKind;
use crate::net::IPv4;
use crate::net::Mask;
use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum CIDRComparison {
    Subset,
    Superset,
    Equals,
    Different,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CIDRParsingError {
    InvalidMaskLength,
    InvalidHostFormat,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CIDR {
    ip: IPv4,
    mask: Mask,
}

impl fmt::Display for CIDR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.ip, self.mask.prefix_length())
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
    pub fn new(ip: IPv4, mask: Mask) -> Self {
        Self { ip, mask }
    }

    pub fn network_address(&self) -> CIDR {
        CIDR {
            ip: self.mask.network_address(&self.ip),
            mask: self.mask,
        }
    }

    pub fn ip(&self) -> IPv4 {
        IPv4::new_from_raw_bytes(self.ip.octets())
    }

    pub fn mask(&self) -> Mask {
        self.mask
    }

    pub fn wildcard_mask(&self) -> Mask {
        self.mask.wildcard()
    }

    pub fn is_network_address(&self) -> bool {
        &self.network_address() == self
    }

    pub fn first_address(&self) -> Option<IPv4> {
        self.mask.first_address(&self.ip)
    }

    pub fn last_address(&self) -> Option<IPv4> {
        self.mask.last_address(&self.ip)
    }

    pub fn broadcast_address(&self) -> Option<IPv4> {
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

    pub fn split(&self, mask: Mask) -> Vec<Self> {
        let base_raw_network_ip = self.network_address().ip.octets();
        let mut networks = Vec::new();

        let mut raw_network_ip = base_raw_network_ip;
        while raw_network_ip < base_raw_network_ip + self.mask.hosts() {
            networks.push(Self::new(IPv4::new_from_raw_bytes(raw_network_ip), mask));
            raw_network_ip += mask.hosts() + 2;
        }

        networks
    }

    fn contains(&self, other: &IPv4) -> bool {
        let start = self.network_address().ip.octets();
        let end = self.broadcast_address().map(|ip| ip.octets());
        if end.is_none() {
            return false;
        }
        let other_ip = other.octets();

        start <= other_ip && other_ip <= end.unwrap()
    }

    pub fn compare(&self, other: &CIDR) -> CIDRComparison {
        let network = self.network_address();
        let other_network = other.network_address();

        if network == other_network {
            CIDRComparison::Equals
        } else if self.mask < other.mask && network.contains(&other.ip()) {
            CIDRComparison::Superset
        } else if other.mask < self.mask && other_network.contains(&self.ip()) {
            CIDRComparison::Subset
        } else {
            CIDRComparison::Different
        }
    }

    pub fn aggregate(&self, mask: Mask) -> CIDR {
        let cidr = Self::new(IPv4::new_from_raw_bytes(self.ip.octets()), mask);
        if mask < self.mask {
            cidr.network_address()
        } else {
            cidr
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CIDRComparison, CIDRParsingError, CIDR};
    use crate::net::{IPClass, IPKind, IPv4, Mask};

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
    fn network_address() {
        let address = CIDR::new(IPv4::new(10, 0, 10, 15), Mask::new(24).unwrap());
        let expected = CIDR::new(IPv4::new(10, 0, 10, 0), Mask::new(24).unwrap());

        assert_eq!(expected, address.network_address())
    }

    #[test]
    fn first_address() {
        let address = CIDR::new(IPv4::new(10, 0, 10, 15), Mask::new(24).unwrap());
        let first_address = IPv4::new(10, 0, 10, 1);

        assert_eq!(Some(first_address), address.first_address())
    }

    #[test]
    fn last_address() {
        let address = CIDR::new(IPv4::new(10, 0, 10, 15), Mask::new(24).unwrap());
        let last_address = IPv4::new(10, 0, 10, 254);

        assert_eq!(Some(last_address), address.last_address())
    }

    #[test]
    fn broadcast_address() {
        let address = CIDR::new(IPv4::new(10, 0, 10, 15), Mask::new(24).unwrap());
        let broadcast_address = IPv4::new(10, 0, 10, 255);

        assert_eq!(Some(broadcast_address), address.broadcast_address())
    }

    #[test]
    fn hosts() {
        let address = CIDR::new(IPv4::new(10, 0, 10, 15), Mask::new(24).unwrap());

        assert_eq!(254, address.hosts())
    }

    #[test]
    fn class() {
        let address = CIDR::new(IPv4::new(10, 0, 10, 15), Mask::new(24).unwrap());

        assert_eq!(IPClass::A, address.class())
    }

    #[test]
    fn kind() {
        let address = CIDR::new(IPv4::new(10, 0, 10, 15), Mask::new(24).unwrap());

        assert_eq!(IPKind::Private, address.kind())
    }

    #[test]
    fn split() {
        let address = CIDR::new(IPv4::new(10, 0, 10, 15), Mask::new(24).unwrap());

        let new_mask = Mask::new(20).unwrap();
        let expected = vec![CIDR::new(IPv4::new(10, 0, 10, 0), Mask::new(20).unwrap())];
        assert_eq!(expected, address.split(new_mask));

        let new_mask = Mask::new(25).unwrap();
        let expected = vec![
            CIDR::new(IPv4::new(10, 0, 10, 0), Mask::new(25).unwrap()),
            CIDR::new(IPv4::new(10, 0, 10, 128), Mask::new(25).unwrap()),
        ];
        assert_eq!(expected, address.split(new_mask));

        let new_mask = Mask::new(26).unwrap();
        let expected = vec![
            CIDR::new(IPv4::new(10, 0, 10, 0), Mask::new(26).unwrap()),
            CIDR::new(IPv4::new(10, 0, 10, 64), Mask::new(26).unwrap()),
            CIDR::new(IPv4::new(10, 0, 10, 128), Mask::new(26).unwrap()),
            CIDR::new(IPv4::new(10, 0, 10, 192), Mask::new(26).unwrap()),
        ];
        assert_eq!(expected, address.split(new_mask));
    }

    #[test]
    fn is_network_address() {
        let host_address = CIDR::new(IPv4::new(10, 0, 10, 15), Mask::new(24).unwrap());
        assert_eq!(false, host_address.is_network_address());

        let network_address = CIDR::new(IPv4::new(10, 0, 10, 0), Mask::new(24).unwrap());
        assert_eq!(true, network_address.is_network_address());
    }

    #[test]
    fn compare() {
        let base_address = CIDR::new(IPv4::new(10, 0, 10, 15), Mask::new(24).unwrap());

        let compared = CIDR::new(IPv4::new(10, 0, 10, 5), Mask::new(24).unwrap());
        assert_eq!(CIDRComparison::Equals, base_address.compare(&compared));

        let compared = CIDR::new(IPv4::new(10, 0, 10, 15), Mask::new(26).unwrap());
        assert_eq!(CIDRComparison::Superset, base_address.compare(&compared));

        let compared = CIDR::new(IPv4::new(10, 0, 10, 5), Mask::new(20).unwrap());
        assert_eq!(CIDRComparison::Subset, base_address.compare(&compared));

        let compared = CIDR::new(IPv4::new(10, 2, 10, 5), Mask::new(24).unwrap());
        assert_eq!(CIDRComparison::Different, base_address.compare(&compared));
    }

    #[test]
    fn aggregate() {
        let base_address = CIDR::new(IPv4::new(10, 0, 10, 248), Mask::new(32).unwrap());

        assert_eq!(
            CIDR::new(IPv4::new(10, 0, 10, 248), Mask::new(31).unwrap()),
            base_address.aggregate(Mask::new(31).unwrap())
        );

        assert_eq!(
            CIDR::new(IPv4::new(10, 0, 10, 240), Mask::new(28).unwrap()),
            base_address.aggregate(Mask::new(28).unwrap())
        );

        assert_eq!(
            CIDR::new(IPv4::new(10, 0, 10, 0), Mask::new(24).unwrap()),
            base_address.aggregate(Mask::new(24).unwrap())
        );

        let base_address = CIDR::new(IPv4::new(10, 0, 10, 248), Mask::new(24).unwrap());
        assert_eq!(
            CIDR::new(IPv4::new(10, 0, 10, 248), Mask::new(28).unwrap()),
            base_address.aggregate(Mask::new(28).unwrap())
        );
    }
}

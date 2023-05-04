use crate::net::IPClass;
use crate::net::IPKind;
use core::fmt;
use std::net;

#[derive(Debug, PartialEq, Eq)]
pub enum IPParsingError {
    InvalidFormat,
}

#[derive(PartialEq, Eq)]
pub struct IPv4(u32);

impl fmt::Debug for IPv4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)?;
        write!(f, " (")?;
        fmt::Binary::fmt(&self, f)?;
        write!(f, ")")?;
        Ok(())
    }
}

impl fmt::Display for IPv4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b, c, d] = crate::net::group_octets(self.0);
        write!(f, "{}.{}.{}.{}", a, b, c, d)
    }
}

impl fmt::Binary for IPv4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b, c, d] = crate::net::group_octets(self.0);
        write!(f, "{:08b}.{:08b}.{:08b}.{:08b}", a, b, c, d)
    }
}

impl std::str::FromStr for IPv4 {
    type Err = IPParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let net_ip = s
            .parse::<net::Ipv4Addr>()
            .map_err(|_| IPParsingError::InvalidFormat)?;

        let octets = net_ip.octets();

        Ok(Self::new(octets[0], octets[1], octets[2], octets[3]))
    }
}

impl IPv4 {
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        let mut addr = (a as u32) << 24;
        addr += (b as u32) << 16;
        addr += (c as u32) << 8;
        addr += d as u32;

        Self(addr)
    }

    pub fn new_from_raw_bytes(addr: u32) -> Self {
        Self(addr)
    }

    pub fn octets(&self) -> u32 {
        self.0
    }

    pub fn class(&self) -> IPClass {
        let octets = self.octets();
        let first_octet = (octets >> 24 & 0xFF) as u8;

        if first_octet < 128 {
            return IPClass::A;
        }

        if first_octet < 192 {
            return IPClass::B;
        }

        if first_octet < 224 {
            return IPClass::C;
        }

        if first_octet < 240 {
            return IPClass::D;
        }

        IPClass::E
    }

    pub fn kind(&self) -> IPKind {
        let octets = self.octets();
        let first_octet = (octets >> 24 & 0xFF) as u8;
        let second_octet = (octets >> 16 & 0xFF) as u8;
        let third_octet = (octets >> 8 & 0xFF) as u8;

        match (first_octet, second_octet, third_octet) {
            (10, _, _) => IPKind::Private,
            (169, 254, _) => IPKind::Special("link-local"),
            (172, 16..=31, _) => IPKind::Private,
            (192, 168, _) => IPKind::Private,
            (192, 0, 2) => IPKind::Special("documentation"),
            (198, 51, 100) => IPKind::Special("documentation"),
            (203, 0, 113) => IPKind::Special("documentation"),
            (127, _, _) => IPKind::Special("localhost"),
            _ => IPKind::Public,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{IPClass, IPKind, IPParsingError, IPv4};

    #[test]
    fn string_display() {
        let ip = IPv4::new(192, 168, 5, 42);
        assert_eq!("192.168.5.42", format!("{}", ip))
    }

    #[test]
    fn binary_display() {
        let ip = IPv4::new(192, 168, 5, 42);
        assert_eq!("11000000.10101000.00000101.00101010", format!("{:b}", ip))
    }

    #[test]
    fn parse_success() {
        let ip = "192.168.13.37".parse::<IPv4>().unwrap();
        assert_eq!(IPv4::new(192, 168, 13, 37), ip)
    }

    #[test]
    fn parse_invalid() {
        assert_eq!(
            Err(IPParsingError::InvalidFormat),
            "512.168.13.37".parse::<IPv4>()
        )
    }

    #[test]
    fn octets() {
        let address = "192.168.5.42".parse::<IPv4>().unwrap();
        assert_eq!(0b11000000101010000000010100101010, address.octets())
    }

    #[test]
    fn class_a() {
        let address = "0.0.0.0".parse::<IPv4>().unwrap();
        assert_eq!(IPClass::A, address.class());

        let address = "127.255.255.255".parse::<IPv4>().unwrap();
        assert_eq!(IPClass::A, address.class());
    }

    #[test]
    fn class_b() {
        let address = "128.0.0.0".parse::<IPv4>().unwrap();
        assert_eq!(IPClass::B, address.class());

        let address = "191.255.255.255".parse::<IPv4>().unwrap();
        assert_eq!(IPClass::B, address.class());
    }

    #[test]
    fn class_c() {
        let address = "192.0.0.0".parse::<IPv4>().unwrap();
        assert_eq!(IPClass::C, address.class());

        let address = "223.255.255.255".parse::<IPv4>().unwrap();
        assert_eq!(IPClass::C, address.class());
    }

    #[test]
    fn class_d() {
        let address = "224.0.0.0".parse::<IPv4>().unwrap();
        assert_eq!(IPClass::D, address.class());

        let address = "239.255.255.255".parse::<IPv4>().unwrap();
        assert_eq!(IPClass::D, address.class());
    }

    #[test]
    fn class_e() {
        let address = "240.0.0.0".parse::<IPv4>().unwrap();
        assert_eq!(IPClass::E, address.class());

        let address = "255.255.255.255".parse::<IPv4>().unwrap();
        assert_eq!(IPClass::E, address.class());
    }

    #[test]
    fn kind_private() {
        let address = "10.0.5.1".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Private, address.kind());
        let address = "10.255.5.1".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Private, address.kind());

        let address = "172.16.5.1".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Private, address.kind());
        let address = "172.31.5.1".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Private, address.kind());

        let address = "192.168.5.1".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Private, address.kind());
    }

    #[test]
    fn kind_public() {
        let address = "9.0.5.1".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Public, address.kind());
        let address = "11.0.5.1".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Public, address.kind());

        let address = "126.255.5.1".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Public, address.kind());
        let address = "128.0.5.1".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Public, address.kind());

        let address = "169.253.5.1".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Public, address.kind());
        let address = "169.255.5.1".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Public, address.kind());

        let address = "172.15.5.1".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Public, address.kind());
        let address = "172.32.5.1".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Public, address.kind());

        let address = "192.167.5.1".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Public, address.kind());
        let address = "192.169.5.1".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Public, address.kind());
    }

    #[test]
    fn kind_special() {
        let address = "127.0.9.1".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Special("localhost"), address.kind());
        let address = "127.255.5.1".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Special("localhost"), address.kind());
        let address = "169.254.5.1".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Special("link-local"), address.kind());
        let address = "192.0.2.1".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Special("documentation"), address.kind());
        let address = "192.0.2.254".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Special("documentation"), address.kind());
        let address = "198.51.100.1".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Special("documentation"), address.kind());
        let address = "198.51.100.254".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Special("documentation"), address.kind());
        let address = "203.0.113.1".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Special("documentation"), address.kind());
        let address = "203.0.113.254".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Special("documentation"), address.kind());
    }
}

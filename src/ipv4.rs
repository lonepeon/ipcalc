use core::fmt;
use std::net;

#[derive(Debug, PartialEq, Eq)]
pub enum IPKind {
    Private,
    Public,
    Special,
}

impl fmt::Display for IPKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            IPKind::Private => "Private Internet",
            IPKind::Public => "Public Internet",
            IPKind::Special => "Special",
        };

        write!(f, "{}", val)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum IPClass {
    A,
    B,
    C,
    D,
    E,
}

impl fmt::Display for IPClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            IPClass::A => "A",
            IPClass::B => "B",
            IPClass::C => "C",
            IPClass::D => "D",
            IPClass::E => "E",
        };

        write!(f, "{}", val)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum IPParsingError {
    InvalidFormat,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IPv4(net::Ipv4Addr);

impl fmt::Display for IPv4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Binary for IPv4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let octets = self.0.octets();
        write!(
            f,
            "{:08b}.{:08b}.{:08b}.{:08b}",
            octets[0], octets[1], octets[2], octets[3]
        )
    }
}

impl std::str::FromStr for IPv4 {
    type Err = IPParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let net_ip = s
            .parse::<net::Ipv4Addr>()
            .map_err(|_| IPParsingError::InvalidFormat)?;

        Ok(Self::new(net_ip))
    }
}

impl IPv4 {
    pub fn new(addr: net::Ipv4Addr) -> Self {
        Self(addr)
    }

    pub fn octets(&self) -> u32 {
        let octets = self.0.octets();

        let mut value = octets[0] as u32;
        value = value << 8 | octets[1] as u32;
        value = value << 8 | octets[2] as u32;
        value = value << 8 | octets[3] as u32;

        value
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

        if first_octet == 10 {
            return IPKind::Private;
        }

        if first_octet == 169 && second_octet == 254 {
            return IPKind::Private;
        }

        if first_octet == 172 && (16..=31).contains(&second_octet) {
            return IPKind::Private;
        }

        if first_octet == 192 && second_octet == 168 {
            return IPKind::Private;
        }

        if first_octet == 127 {
            return IPKind::Special;
        }

        IPKind::Public
    }
}

#[cfg(test)]
mod tests {
    use super::{IPClass, IPKind, IPParsingError, IPv4};
    use std::net;

    #[test]
    fn string_display() {
        let ip = IPv4::new(net::Ipv4Addr::new(192, 168, 5, 42));
        assert_eq!("192.168.5.42", format!("{}", ip))
    }

    #[test]
    fn binary_display() {
        let ip = IPv4::new(net::Ipv4Addr::new(192, 168, 5, 42));
        assert_eq!("11000000.10101000.00000101.00101010", format!("{:b}", ip))
    }

    #[test]
    fn parse_success() {
        let ip = "192.168.13.37".parse::<IPv4>().unwrap();
        assert_eq!(IPv4::new(net::Ipv4Addr::new(192, 168, 13, 37)), ip)
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

        let address = "169.254.5.1".parse::<IPv4>().unwrap();
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
        assert_eq!(IPKind::Special, address.kind());
        let address = "127.255.5.1".parse::<IPv4>().unwrap();
        assert_eq!(IPKind::Special, address.kind());
    }
}

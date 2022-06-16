use core::fmt;
use std::net;

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
}

#[cfg(test)]
mod tests {
    use super::{IPParsingError, IPv4};
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
}

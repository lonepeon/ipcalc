mod ipv4;
mod mask;

use ipv4::IPv4;
use mask::Mask;
use std::net;

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

impl CIDR {
    pub fn parse(raw: &str) -> Result<Self, CIDRParsingError> {
        let values: Vec<&str> = raw.split('/').collect();
        if values.len() > 2 {
            return Err(CIDRParsingError::InvalidHostFormat);
        }

        let ip = values[0]
            .parse::<net::Ipv4Addr>()
            .map_err(|_| CIDRParsingError::InvalidHostFormat)?;

        let mask = if values.len() == 2 {
            values[1]
                .parse::<Mask>()
                .map_err(|_| CIDRParsingError::InvalidMaskLength)?
        } else {
            Mask::new(32).unwrap()
        };

        Ok(Self {
            ip: IPv4::new(ip),
            mask,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{CIDRParsingError, CIDR};

    #[test]
    fn parse_mask_negative() {
        assert_eq!(
            Err(CIDRParsingError::InvalidMaskLength),
            CIDR::parse(&"10.0.0.0/-5".to_string())
        )
    }

    #[test]
    fn parse_mask_zero() {
        assert_eq!(
            Err(CIDRParsingError::InvalidMaskLength),
            CIDR::parse(&"10.0.0.0/0".to_string())
        )
    }

    #[test]
    fn parse_mask_too_small() {
        assert_eq!(
            Err(CIDRParsingError::InvalidMaskLength),
            CIDR::parse(&"10.0.0.0/38".to_string())
        )
    }

    #[test]
    fn parse_ip_missing_bytes() {
        assert_eq!(
            Err(CIDRParsingError::InvalidHostFormat),
            CIDR::parse(&"10.0/20".to_string())
        )
    }

    #[test]
    fn parse_ip_contains_several_slashes() {
        assert_eq!(
            Err(CIDRParsingError::InvalidHostFormat),
            CIDR::parse(&"10.0.10.0/24/12".to_string())
        )
    }
}

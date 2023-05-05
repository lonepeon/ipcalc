use crate::cli::ErrorKind;
use crate::net::{CIDRParsingError, Mask, MaskParsingError, CIDR};

pub fn parse_cidr(name: &str, raw: String) -> Result<CIDR, ErrorKind> {
    if raw.is_empty() {
        return Err(ErrorKind::InvalidInput(format!(
            "expecting non empty {} argument",
            name
        )));
    }

    match raw.parse::<CIDR>() {
        Ok(cidr) => Ok(cidr),
        Err(CIDRParsingError::InvalidMaskLength) => Err(ErrorKind::InvalidInput(
            "masklength must be between 0 and 32".to_string(),
        )),
        Err(CIDRParsingError::InvalidHostFormat) => Err(ErrorKind::InvalidInput(
            "invalid IPv4 CIDR format".to_string(),
        )),
    }
}

pub fn parse_mask(name: &str, raw: String) -> Result<Mask, ErrorKind> {
    if raw.is_empty() {
        return Err(ErrorKind::InvalidInput(format!(
            "expecting non empty {} argument",
            name
        )));
    }

    match raw.parse::<Mask>() {
        Ok(mask) => Ok(mask),
        Err(MaskParsingError::InvalidRange) => Err(ErrorKind::InvalidInput(
            "mask length must be between 0 and 32".to_string(),
        )),
        Err(MaskParsingError::InvalidFormat) => {
            Err(ErrorKind::InvalidInput("invalid mask format".to_string()))
        }
    }
}

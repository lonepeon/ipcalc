use crate::cli::ErrorKind;
use crate::net::{CIDRParsingError, CIDR};

pub fn parser(raw: String) -> Result<CIDR, ErrorKind> {
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

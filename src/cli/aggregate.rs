use crate::cli::ErrorKind;
use crate::net::{CIDRParsingError, Mask, MaskParsingError, CIDR};

pub struct CLI<W: std::io::Write> {
    pub out: W,
}

impl<W: std::io::Write> CLI<W> {
    pub fn new(out: W) -> Self {
        CLI { out }
    }

    pub fn execute(&mut self, raw_cidr: String, raw_mask: String) -> Result<(), ErrorKind> {
        if raw_cidr.is_empty() {
            return Err(ErrorKind::InvalidInput(
                "expecting non empty CIDR argument".to_string(),
            ));
        }

        if raw_mask.is_empty() {
            return Err(ErrorKind::InvalidInput(
                "expecting non empty MASK argument".to_string(),
            ));
        }

        let mask = match raw_mask.parse::<Mask>() {
            Ok(mask) => mask,
            Err(MaskParsingError::InvalidRange) => {
                return Err(ErrorKind::InvalidInput(
                    "invalid mask range value".to_string(),
                ))
            }
            Err(MaskParsingError::InvalidFormat) => {
                return Err(ErrorKind::InvalidInput("invalid mask format".to_string()))
            }
        };

        let cidr = match raw_cidr.parse::<CIDR>() {
            Ok(cidr) => cidr,
            Err(CIDRParsingError::InvalidMaskLength) => {
                return Err(ErrorKind::InvalidInput(
                    "IPv4 CIDR masklength must be between 0 and 32".to_string(),
                ));
            }
            Err(CIDRParsingError::InvalidHostFormat) => {
                return Err(ErrorKind::InvalidInput(
                    "invalid IPv4 CIDR format".to_string(),
                ))
            }
        };

        let start = mask.prefix_length().min(cidr.mask().prefix_length());
        let stop = mask.prefix_length().max(cidr.mask().prefix_length());
        for mask_length in start..=stop {
            let mask = Mask::new(mask_length).expect("{} is not a valid mask length");
            writeln!(self.out, "{}", cidr.aggregate(mask)).unwrap();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn aggregate_32_to_24() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        cli.execute("10.12.5.255/32".to_string(), "24".to_string())
            .unwrap();

        let expected_output = include_str!("testdata/aggregate-32-to-24.golden");
        let actual_output = String::from_utf8(output).unwrap();

        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn aggregate_24_to_32() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        cli.execute("10.12.5.255/24".to_string(), "32".to_string())
            .unwrap();

        let expected_output = include_str!("testdata/aggregate-24-to-32.golden");
        let actual_output = String::from_utf8(output).unwrap();

        assert_eq!(expected_output, actual_output);
    }
}

use crate::cli::cidr_formatter::CIDFormatter;
use crate::cli::ErrorKind;
use crate::net::{CIDRParsingError, CIDR};
use crate::net::{Mask, MaskParsingError};

pub struct CLI<W: std::io::Write> {
    pub out: W,
    pub with_binary: bool,
}

impl<W: std::io::Write> CLI<W> {
    pub fn new(out: W) -> Self {
        CLI {
            out,
            with_binary: true,
        }
    }

    pub fn execute(&mut self, raw_cidr: String, raw_new_mask: String) -> Result<(), ErrorKind> {
        if raw_cidr.is_empty() {
            return Err(ErrorKind::InvalidInput(
                "expecting non empty CIDR argument".to_string(),
            ));
        }

        if raw_new_mask.is_empty() {
            return Err(ErrorKind::InvalidInput(
                "expecting non empty MASK argument".to_string(),
            ));
        }

        let new_mask = match raw_new_mask.parse::<Mask>() {
            Ok(mask) => mask,
            Err(MaskParsingError::InvalidRange) => {
                return Err(ErrorKind::InvalidInput(
                    "invalid split mask range value".to_string(),
                ))
            }
            Err(MaskParsingError::InvalidFormat) => {
                return Err(ErrorKind::InvalidInput(
                    "invalid split mask format".to_string(),
                ))
            }
        };

        let cidr = match raw_cidr.parse::<CIDR>() {
            Ok(cidr) => cidr,
            Err(CIDRParsingError::InvalidMaskLength) => {
                return Err(ErrorKind::InvalidInput(
                    "masklength must be between 0 and 32".to_string(),
                ));
            }
            Err(CIDRParsingError::InvalidHostFormat) => {
                return Err(ErrorKind::InvalidInput(
                    "invalid IPv4 CIDR format".to_string(),
                ))
            }
        };

        if !cidr.is_network_address() {
            write!(
                self.out,
                "{}",
                CIDFormatter {
                    cidr: CIDR::new(cidr.ip(), new_mask),
                    with_binary: self.with_binary
                }
            )
            .unwrap();
            return Ok(());
        }

        for cidr in cidr.split(new_mask) {
            writeln!(self.out, "========").unwrap();

            write!(
                self.out,
                "{}",
                CIDFormatter {
                    cidr,
                    with_binary: self.with_binary
                }
            )
            .unwrap();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn split_host_slash_24_to_26() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        cli.execute("10.13.5.78/24".to_string(), "26".to_string())
            .unwrap();

        let expected_output = fs::read_to_string("src/cli/testdata/split-host.golden").unwrap();
        let actual_output = String::from_utf8(output).unwrap();

        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn split_host_slash_24_to_26_no_binary() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        cli.with_binary = false;
        cli.execute("10.13.5.78/24".to_string(), "26".to_string())
            .unwrap();

        let expected_output =
            fs::read_to_string("src/cli/testdata/split-host-no-binary.golden").unwrap();
        let actual_output = String::from_utf8(output).unwrap();

        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn split_network_slash_24_to_26() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        cli.execute("10.13.5.0/24".to_string(), "26".to_string())
            .unwrap();

        let expected_output = fs::read_to_string("src/cli/testdata/split-network.golden").unwrap();
        let actual_output = String::from_utf8(output).unwrap();

        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn split_network_slash_24_to_26_no_binary() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        cli.with_binary = false;
        cli.execute("10.13.5.0/24".to_string(), "26".to_string())
            .unwrap();

        let expected_output =
            fs::read_to_string("src/cli/testdata/split-network-no-binary.golden").unwrap();
        let actual_output = String::from_utf8(output).unwrap();

        assert_eq!(expected_output, actual_output);
    }
}

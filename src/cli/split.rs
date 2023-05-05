use crate::cli::cidr_formatter::CIDRFormatter;
use crate::cli::ErrorKind;
use crate::net::CIDR;

use super::arg_parser;

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
        let cidr = arg_parser::parse_cidr("CIDR", raw_cidr)?;
        let new_mask = arg_parser::parse_mask("MASK", raw_new_mask)?;

        if !cidr.is_network_address() {
            write!(
                self.out,
                "{}",
                CIDRFormatter {
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
                CIDRFormatter {
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

    use crate::cli::ErrorKind;

    #[test]
    fn split_empty_cidr() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        let output = cli.execute("".to_string(), "24".to_string());

        assert_eq!(
            Err(ErrorKind::InvalidInput(
                "expecting non empty CIDR argument".to_string()
            )),
            output
        );
    }

    #[test]
    fn split_empty_mask() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        let output = cli.execute("10.12.5.255/32".to_string(), "".to_string());

        assert_eq!(
            Err(ErrorKind::InvalidInput(
                "expecting non empty MASK argument".to_string()
            )),
            output
        );
    }

    #[test]
    fn split_unparsable_cidr() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        let output = cli.execute("not a CIDR".to_string(), "24".to_string());

        assert_eq!(
            Err(ErrorKind::InvalidInput(
                "invalid IPv4 CIDR format".to_string()
            )),
            output
        );
    }

    #[test]
    fn split_invalid_cidr_mask() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        let output = cli.execute("10.12.5.255/200".to_string(), "24".to_string());

        assert_eq!(
            Err(ErrorKind::InvalidInput(
                "masklength must be between 0 and 32".to_string()
            )),
            output
        );
    }

    #[test]
    fn split_unparsable_mask() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        let output = cli.execute("10.12.5.255/32".to_string(), "not a mask".to_string());

        assert_eq!(
            Err(ErrorKind::InvalidInput("invalid mask format".to_string())),
            output
        );
    }

    #[test]
    fn split_invalid_mask_range() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        let output = cli.execute("10.12.5.255/32".to_string(), "200".to_string());

        assert_eq!(
            Err(ErrorKind::InvalidInput(
                "mask length must be between 0 and 32".to_string()
            )),
            output
        );
    }

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

use crate::cli::cidr_formatter::CIDRFormatter;
use crate::cli::ErrorKind;

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

    pub fn execute(&mut self, raw_cidr: String) -> Result<(), ErrorKind> {
        let cidr = arg_parser::parse_cidr("CIDR", raw_cidr)?;
        let formatter = CIDRFormatter {
            cidr,
            with_binary: self.with_binary,
        };

        write!(self.out, "{}", formatter).unwrap();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::ErrorKind;

    #[test]
    fn describe_empty_cidr() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        let output = cli.execute("".to_string());

        assert_eq!(
            Err(ErrorKind::InvalidInput(
                "expecting non empty CIDR argument".to_string()
            )),
            output
        );
    }

    #[test]
    fn describe_unparsable_cidr() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        let output = cli.execute("not a CIDR".to_string());

        assert_eq!(
            Err(ErrorKind::InvalidInput(
                "invalid IPv4 CIDR format".to_string()
            )),
            output
        );
    }

    #[test]
    fn describe_invalid_mask() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        let output = cli.execute("10.12.23.43/200".to_string());

        assert_eq!(
            Err(ErrorKind::InvalidInput(
                "masklength must be between 0 and 32".to_string()
            )),
            output
        );
    }

    #[test]
    fn describe_class_a() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        cli.execute("10.12.23.43/20".to_string()).unwrap();

        let expected_output = include_str!("testdata/describe.golden");
        let actual_output = String::from_utf8(output).unwrap();

        assert_eq!(expected_output, actual_output);
    }
}

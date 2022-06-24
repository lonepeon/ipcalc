use crate::cli::cidr_describer::CIDRDescriber;
use crate::cli::ErrorKind;
use crate::net::{CIDRParsingError, CIDR};

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
        if raw_cidr.is_empty() {
            return Err(ErrorKind::InvalidInput("expecting an argument".to_string()));
        }

        match raw_cidr.parse::<CIDR>() {
            Ok(cidr) => {
                write!(
                    self.out,
                    "{}",
                    CIDRDescriber {
                        cidr,
                        with_binary: self.with_binary
                    }
                )
                .unwrap();

                Ok(())
            }
            Err(CIDRParsingError::InvalidMaskLength) => Err(ErrorKind::InvalidInput(
                "masklength must be between 0 and 32".to_string(),
            )),
            Err(CIDRParsingError::InvalidHostFormat) => Err(ErrorKind::InvalidInput(
                "invalid IPv4 CIDR format".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn describe_class_a() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        cli.execute("10.12.23.43/20".to_string()).unwrap();

        let expected_output = fs::read_to_string("src/cli/testdata/describe.golden").unwrap();
        let actual_output = String::from_utf8(output).unwrap();

        assert_eq!(expected_output, actual_output);
    }
}

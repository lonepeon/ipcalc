use crate::cli::ErrorKind;
use crate::net::Mask;

use super::arg_parser;

pub struct CLI<W: std::io::Write> {
    pub out: W,
}

impl<W: std::io::Write> CLI<W> {
    pub fn new(out: W) -> Self {
        CLI { out }
    }

    pub fn execute(&mut self, raw_cidr: String, raw_mask: String) -> Result<(), ErrorKind> {
        let cidr = arg_parser::parse_cidr("CIDR", raw_cidr)?;
        let mask = arg_parser::parse_mask("MASK", raw_mask)?;

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
    use crate::cli::ErrorKind;

    #[test]
    fn aggregate_empty_cidr() {
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
    fn aggregate_empty_mask() {
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
    fn aggregate_unparsable_cidr() {
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
    fn aggregate_invalid_cidr_mask() {
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
    fn aggregate_unparsable_mask() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        let output = cli.execute("10.12.5.255/32".to_string(), "not a mask".to_string());

        assert_eq!(
            Err(ErrorKind::InvalidInput("invalid mask format".to_string())),
            output
        );
    }

    #[test]
    fn aggregate_invalid_mask_range() {
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

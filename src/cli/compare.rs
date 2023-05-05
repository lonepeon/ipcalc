use crate::cli::arg_parser;
use crate::cli::ErrorKind;
use crate::net::CIDRComparison;

pub struct CLI<W: std::io::Write> {
    pub out: W,
}

impl<W: std::io::Write> CLI<W> {
    pub fn new(out: W) -> Self {
        CLI { out }
    }

    pub fn execute(&mut self, raw_cidr: String, raw_other: String) -> Result<(), ErrorKind> {
        let cidr = arg_parser::parse_cidr("CIDR", raw_cidr)?;
        let other = arg_parser::parse_cidr("OTHER_CIDR", raw_other)?;
        let description = match cidr.compare(&other) {
            CIDRComparison::Subset => "is a subset of",
            CIDRComparison::Superset => "is a superset of",
            CIDRComparison::Equals => "is the same as",
            CIDRComparison::Different => "is in a different network than",
        };

        writeln!(self.out, "{} {} {}", cidr, description, other,).unwrap();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::ErrorKind;

    #[test]
    fn compare_empty_cidr1() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        let output = cli.execute("".to_string(), "10.12.25.99/24".to_string());

        assert_eq!(
            Err(ErrorKind::InvalidInput(
                "expecting non empty CIDR argument".to_string()
            )),
            output
        );
    }

    #[test]
    fn compare_unparsable_cidr1() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        let output = cli.execute("not a CIDR".to_string(), "10.12.25.99/24".to_string());

        assert_eq!(
            Err(ErrorKind::InvalidInput(
                "invalid IPv4 CIDR format".to_string()
            )),
            output
        );
    }

    #[test]
    fn compare_invalid_mask_cidr1() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        let output = cli.execute("10.12.23.43/200".to_string(), "10.12.25.99/24".to_string());

        assert_eq!(
            Err(ErrorKind::InvalidInput(
                "masklength must be between 0 and 32".to_string()
            )),
            output
        );
    }

    #[test]
    fn compare_empty_cidr2() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        let output = cli.execute("10.12.23.43/16".to_string(), "".to_string());

        assert_eq!(
            Err(ErrorKind::InvalidInput(
                "expecting non empty OTHER_CIDR argument".to_string()
            )),
            output
        );
    }

    #[test]
    fn compare_unparsable_cidr2() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        let output = cli.execute("10.12.23.43/16".to_string(), "not a CIDR".to_string());

        assert_eq!(
            Err(ErrorKind::InvalidInput(
                "invalid IPv4 CIDR format".to_string()
            )),
            output
        );
    }
    #[test]
    fn compare_invalid_mask_cidr2() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        let output = cli.execute("10.12.23.43/16".to_string(), "10.12.25.99/200".to_string());

        assert_eq!(
            Err(ErrorKind::InvalidInput(
                "masklength must be between 0 and 32".to_string()
            )),
            output
        );
    }

    #[test]
    fn on_superset() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        cli.execute("10.12.23.43/16".to_string(), "10.12.25.99/24".to_string())
            .unwrap();

        let expected_output = include_str!("testdata/compare-superset.golden");
        let actual_output = String::from_utf8(output).unwrap();

        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn on_subset() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        cli.execute("10.12.23.43/24".to_string(), "10.12.25.99/16".to_string())
            .unwrap();

        let expected_output = include_str!("testdata/compare-subset.golden");
        let actual_output = String::from_utf8(output).unwrap();

        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn on_equals() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        cli.execute("10.12.23.43/16".to_string(), "10.12.25.99/16".to_string())
            .unwrap();

        let expected_output = include_str!("testdata/compare-equals.golden");
        let actual_output = String::from_utf8(output).unwrap();

        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn on_different() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        cli.execute("10.12.23.43/16".to_string(), "10.14.25.99/16".to_string())
            .unwrap();

        let expected_output = include_str!("testdata/compare-different.golden");
        let actual_output = String::from_utf8(output).unwrap();

        assert_eq!(expected_output, actual_output);
    }
}

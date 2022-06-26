use crate::cli::cidr_parser;
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
        if raw_cidr.is_empty() {
            return Err(ErrorKind::InvalidInput("expecting an argument".to_string()));
        }

        if raw_other.is_empty() {
            return Err(ErrorKind::InvalidInput("expecting an argument".to_string()));
        }

        let cidr = cidr_parser::parser(raw_cidr)?;
        let other = cidr_parser::parser(raw_other)?;
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
    use std::fs;

    #[test]
    fn on_superset() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        cli.execute("10.12.23.43/16".to_string(), "10.12.25.99/24".to_string())
            .unwrap();

        let expected_output =
            fs::read_to_string("src/cli/testdata/compare-superset.golden").unwrap();
        let actual_output = String::from_utf8(output).unwrap();

        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn on_subset() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        cli.execute("10.12.23.43/24".to_string(), "10.12.25.99/16".to_string())
            .unwrap();

        let expected_output = fs::read_to_string("src/cli/testdata/compare-subset.golden").unwrap();
        let actual_output = String::from_utf8(output).unwrap();

        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn on_equals() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        cli.execute("10.12.23.43/16".to_string(), "10.12.25.99/16".to_string())
            .unwrap();

        let expected_output = fs::read_to_string("src/cli/testdata/compare-equals.golden").unwrap();
        let actual_output = String::from_utf8(output).unwrap();

        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn on_different() {
        let mut output = Vec::new();
        let mut cli = super::CLI::new(&mut output);
        cli.execute("10.12.23.43/16".to_string(), "10.14.25.99/16".to_string())
            .unwrap();

        let expected_output =
            fs::read_to_string("src/cli/testdata/compare-different.golden").unwrap();
        let actual_output = String::from_utf8(output).unwrap();

        assert_eq!(expected_output, actual_output);
    }
}

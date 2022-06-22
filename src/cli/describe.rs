use crate::net::{CIDRParsingError, CIDR};

use crate::cli::ErrorKind;

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

    fn write<D: std::fmt::Display>(&mut self, d: D) {
        write!(&mut self.out, "{}", d).unwrap();
    }

    fn write_if_binary<D: std::fmt::Display>(&mut self, d: D) {
        if !self.with_binary {
            return;
        }
        write!(&mut self.out, "{}", d).unwrap();
    }

    pub fn execute(&mut self, args: Vec<String>) -> Result<(), ErrorKind> {
        if args.len() != 1 {
            return Err(ErrorKind::InvalidInput(
                "expecting only one CIDR as argument".to_string(),
            ));
        }

        match args[0].parse::<CIDR>() {
            Ok(cidr) => {
                self.write(format!("Address:   {:15}      ", format!("{}", cidr.ip())));
                self.write_if_binary(format!("{:b}", cidr.ip()));
                self.write("\n");

                self.write(format!(
                    "Netmask:   {:20} ",
                    format!("{} = {}", cidr.mask(), cidr.mask().len())
                ));
                self.write_if_binary(format!("{:b}", cidr.mask()));
                self.write("\n");

                self.write(format!(
                    "Wildcard:  {:15}      ",
                    format!("{}", cidr.wildcard_mask())
                ));
                self.write_if_binary(format!("{:b}", cidr.wildcard_mask()));
                self.write("\n");

                self.write("=>\n");

                self.write(format!(
                    "Network:   {:18}   ",
                    format!("{}", cidr.network())
                ));
                self.write_if_binary(format!("{:b}", cidr.network().ip()));
                self.write("\n");

                self.write(format!(
                    "HostMin:   {:18}   ",
                    cidr.first_address()
                        .map(|ip| format!("{}", ip))
                        .unwrap_or_else(|| "n/a".to_string()),
                ));
                self.write_if_binary(
                    cidr.first_address()
                        .map(|ip| format!("{:b}", ip))
                        .unwrap_or_else(|| "".to_string()),
                );
                self.write("\n");

                self.write(format!(
                    "HostMax:   {:18}   ",
                    cidr.last_address()
                        .map(|ip| format!("{}", ip))
                        .unwrap_or_else(|| "n/a".to_string()),
                ));
                self.write_if_binary(
                    cidr.last_address()
                        .map(|ip| format!("{:b}", ip))
                        .unwrap_or_else(|| "".to_string()),
                );
                self.write("\n");

                self.write(format!(
                    "Broadcast: {:18}   ",
                    cidr.broadcast_address()
                        .map(|ip| format!("{}", ip))
                        .unwrap_or_else(|| "n/a".to_string()),
                ));
                self.write_if_binary(
                    cidr.broadcast_address()
                        .map(|ip| format!("{:b}", ip))
                        .unwrap_or_else(|| "".to_string()),
                );
                self.write("\n");

                self.write(format!(
                    "Hosts/Net: {:10}           class {}, {}",
                    format!("{}", cidr.hosts()),
                    cidr.class(),
                    cidr.kind(),
                ));
                self.write("\n");

                Ok(())
            }
            Err(CIDRParsingError::InvalidMaskLength) => Err(ErrorKind::InvalidInput(
                "masklength must be between 0 and 32".to_string(),
            )),
            Err(CIDRParsingError::InvalidHostFormat) => Err(ErrorKind::InvalidInput(
                "host must use aaa.bbb.ccc.ddd format".to_string(),
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
        cli.execute(vec!["10.12.23.43/20".to_string()]).unwrap();

        let expected_output = fs::read_to_string("src/cli/testdata/describe.golden").unwrap();
        let actual_output = String::from_utf8(output).unwrap();

        assert_eq!(expected_output, actual_output);
    }
}

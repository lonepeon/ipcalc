use ipcalc::{CIDRParsingError, CIDR};

fn main() {
    match CIDR::parse("10.0.0.0/24") {
        Ok(cidr) => println!("{:?}", cidr),
        Err(CIDRParsingError::InvalidMaskLength) => println!("masklength must be between 1 and 32"),
        Err(CIDRParsingError::InvalidHostFormat) => {
            println!("host must use aaa.bbb.ccc.ddd format")
        }
    }
}

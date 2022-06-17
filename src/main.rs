use ipcalc::{CIDRParsingError, CIDR};

fn main() {
    match CIDR::parse("10.0.0.0/24") {
        Ok(cidr) => {
            println!("Address: {:15}      {:b}", cidr.ip, cidr.ip);
            println!(
                "Netmask: {:20} {:b}",
                format!("{} = {}", cidr.mask, cidr.mask.len()),
                cidr.mask,
            );
            println!(
                "Wildcard: {:15}     {:b}",
                cidr.mask.wildcard(),
                cidr.mask.wildcard(),
            );
        }
        Err(CIDRParsingError::InvalidMaskLength) => println!("masklength must be between 1 and 32"),
        Err(CIDRParsingError::InvalidHostFormat) => {
            println!("host must use aaa.bbb.ccc.ddd format")
        }
    }
}

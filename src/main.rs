use ipcalc::{CIDRParsingError, CIDR};

fn main() {
    match "10.0.22.12/20".parse::<CIDR>() {
        Ok(cidr) => {
            println!("Address:  {:15}      {:b}", cidr.ip, cidr.ip);
            println!(
                "Netmask:  {:20} {:b}",
                format!("{} = {}", cidr.mask, cidr.mask.len()),
                cidr.mask,
            );
            println!(
                "Wildcard: {:15}      {:b}",
                format!("{}", cidr.mask.wildcard()),
                cidr.mask.wildcard(),
            );
            println!("=>");
            println!(
                "Network:  {:18}   {:b}",
                format!("{}", cidr.network()),
                cidr.network().ip
            )
        }
        Err(CIDRParsingError::InvalidMaskLength) => println!("masklength must be between 0 and 32"),
        Err(CIDRParsingError::InvalidHostFormat) => {
            println!("host must use aaa.bbb.ccc.ddd format")
        }
    }
}

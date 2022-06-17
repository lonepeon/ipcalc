use ipcalc::{CIDRParsingError, CIDR};

fn main() {
    match "10.0.22.12/20".parse::<CIDR>() {
        Ok(cidr) => {
            println!("Address:   {:15}      {:b}", cidr.ip, cidr.ip);
            println!(
                "Netmask:   {:20} {:b}",
                format!("{} = {}", cidr.mask, cidr.mask.len()),
                cidr.mask,
            );
            println!(
                "Wildcard:  {:15}      {:b}",
                format!("{}", cidr.mask.wildcard()),
                cidr.mask.wildcard(),
            );

            println!("=>");

            println!(
                "Network:   {:18}   {:b}",
                format!("{}", cidr.network()),
                cidr.network().ip
            );
            println!(
                "HostMin:   {:18}   {:b}",
                format!("{}", cidr.first_address()),
                cidr.first_address()
            );
            println!(
                "HostMax:   {:18}   {:b}",
                format!("{}", cidr.last_address()),
                cidr.last_address()
            );
            println!(
                "Broadcast: {:18}   {:b}",
                format!("{}", cidr.broadcast_address()),
                cidr.broadcast_address()
            )
        }
        Err(CIDRParsingError::InvalidMaskLength) => println!("masklength must be between 0 and 32"),
        Err(CIDRParsingError::InvalidHostFormat) => {
            println!("host must use aaa.bbb.ccc.ddd format")
        }
    }
}

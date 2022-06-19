use ipcalc::{CIDRParsingError, CIDR};

fn main() {
    match "10.0.22.12/20".parse::<CIDR>() {
        Ok(cidr) => {
            println!(
                "Address:   {:15}      {:b}",
                format!("{}", cidr.ip()),
                cidr.ip()
            );
            println!(
                "Netmask:   {:20} {:b}",
                format!("{} = {}", cidr.mask(), cidr.mask().len()),
                cidr.mask(),
            );
            println!(
                "Wildcard:  {:15}      {:b}",
                format!("{}", cidr.wildcard_mask()),
                cidr.wildcard_mask(),
            );

            println!("=>");

            println!(
                "Network:   {:18}   {:b}",
                format!("{}", cidr.network()),
                cidr.network().ip(),
            );
            println!(
                "HostMin:   {:18}   {}",
                cidr.first_address()
                    .map(|ip| format!("{}", ip))
                    .unwrap_or_else(|| "n/a".to_string()),
                cidr.first_address()
                    .map(|ip| format!("{:b}", ip))
                    .unwrap_or_else(|| "".to_string()),
            );
            println!(
                "HostMax:   {:18}   {}",
                cidr.last_address()
                    .map(|ip| format!("{}", ip))
                    .unwrap_or_else(|| "n/a".to_string()),
                cidr.last_address()
                    .map(|ip| format!("{:b}", ip))
                    .unwrap_or_else(|| "".to_string()),
            );
            println!(
                "Broadcast: {:18}   {}",
                cidr.broadcast_address()
                    .map(|ip| format!("{}", ip))
                    .unwrap_or_else(|| "n/a".to_string()),
                cidr.broadcast_address()
                    .map(|ip| format!("{:b}", ip))
                    .unwrap_or_else(|| "".to_string()),
            );
            println!(
                "Hosts/Net: {:10}           class {}, {}",
                format!("{}", cidr.hosts()),
                cidr.class(),
                cidr.kind(),
            )
        }
        Err(CIDRParsingError::InvalidMaskLength) => println!("masklength must be between 0 and 32"),
        Err(CIDRParsingError::InvalidHostFormat) => {
            println!("host must use aaa.bbb.ccc.ddd format")
        }
    }
}

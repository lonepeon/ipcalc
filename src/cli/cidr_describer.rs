use crate::net::CIDR;
use core::fmt;

pub struct CIDRDescriber {
    pub cidr: CIDR,
    pub with_binary: bool,
}

impl fmt::Display for CIDRDescriber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address:   {:15}      ", format!("{}", self.cidr.ip()))?;
        if self.with_binary {
            write!(f, "{:b}", self.cidr.ip())?;
        }
        writeln!(f)?;

        write!(
            f,
            "Netmask:   {:20} ",
            format!("{} = {}", self.cidr.mask(), self.cidr.mask().len())
        )?;
        if self.with_binary {
            write!(f, "{:b}", self.cidr.mask())?;
        }
        writeln!(f)?;

        write!(
            f,
            "Wildcard:  {:15}      ",
            format!("{}", self.cidr.wildcard_mask())
        )?;
        if self.with_binary {
            write!(f, "{:b}", self.cidr.wildcard_mask())?;
        }
        writeln!(f)?;

        writeln!(f, "=>")?;

        write!(f, "Network:   {:18}   ", format!("{}", self.cidr.network()))?;
        if self.with_binary {
            write!(f, "{:b}", self.cidr.network().ip())?;
        }
        writeln!(f)?;

        write!(
            f,
            "HostMin:   {:18}   ",
            self.cidr
                .first_address()
                .map(|ip| format!("{}", ip))
                .unwrap_or_else(|| "n/a".to_string()),
        )?;
        if self.with_binary {
            self.cidr.first_address().map(|ip| write!(f, "{:b}", ip));
        }
        writeln!(f)?;

        write!(
            f,
            "HostMax:   {:18}   ",
            self.cidr
                .last_address()
                .map(|ip| format!("{}", ip))
                .unwrap_or_else(|| "n/a".to_string()),
        )?;
        if self.with_binary {
            self.cidr.last_address().map(|ip| write!(f, "{:b}", ip));
        }
        writeln!(f)?;

        write!(
            f,
            "Broadcast: {:18}   ",
            self.cidr
                .broadcast_address()
                .map(|ip| format!("{}", ip))
                .unwrap_or_else(|| "n/a".to_string()),
        )?;
        if self.with_binary {
            self.cidr
                .broadcast_address()
                .map(|ip| write!(f, "{:b}", ip));
        }
        writeln!(f)?;

        write!(
            f,
            "Hosts/Net: {:10}           class {}, {}",
            format!("{}", self.cidr.hosts()),
            self.cidr.class(),
            self.cidr.kind(),
        )?;

        writeln!(f)
    }
}

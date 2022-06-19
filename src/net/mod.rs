mod cidr;
mod ipclass;
mod ipkind;
mod ipv4;
mod mask;

pub use self::cidr::{CIDRParsingError, CIDR};
pub use self::ipclass::IPClass;
pub use self::ipkind::IPKind;
pub use self::ipv4::{IPParsingError, IPv4};
pub use self::mask::{Mask, MaskParsingError};

pub fn group_octets(value: u32) -> [u8; 4] {
    let a = (value >> 24 & 0xFF) as u8;
    let b = (value >> 16 & 0xFF) as u8;
    let c = (value >> 8 & 0xFF) as u8;
    let d = (value & 0xFF) as u8;

    [a, b, c, d]
}

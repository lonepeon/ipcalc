use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum IPKind {
    Private,
    Public,
    Special,
}

impl fmt::Display for IPKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            IPKind::Private => "Private Internet",
            IPKind::Public => "Public Internet",
            IPKind::Special => "Special",
        };

        write!(f, "{}", val)
    }
}

use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum IPKind {
    Private,
    Public,
    Special(&'static str),
}

impl fmt::Display for IPKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IPKind::Private => write!(f, "Private Internet"),
            IPKind::Public => write!(f, "Public Internet"),
            IPKind::Special(kind) => write!(f, "Special ({})", kind),
        }
    }
}

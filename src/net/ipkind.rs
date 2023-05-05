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

#[cfg(test)]
mod tests {
    #[test]
    fn kind_private() {
        assert_eq!("Private Internet", format!("{}", super::IPKind::Private))
    }

    #[test]
    fn kind_public() {
        assert_eq!("Public Internet", format!("{}", super::IPKind::Public))
    }

    #[test]
    fn kind_special() {
        assert_eq!(
            "Special (something)",
            format!("{}", super::IPKind::Special("something"))
        )
    }
}

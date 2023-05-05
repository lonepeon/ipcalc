use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum IPClass {
    A,
    B,
    C,
    D,
    E,
}

impl fmt::Display for IPClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            IPClass::A => "A",
            IPClass::B => "B",
            IPClass::C => "C",
            IPClass::D => "D",
            IPClass::E => "E",
        };

        write!(f, "{}", val)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn class_a() {
        assert_eq!("A", format!("{}", super::IPClass::A))
    }

    #[test]
    fn class_b() {
        assert_eq!("B", format!("{}", super::IPClass::B))
    }

    #[test]
    fn class_c() {
        assert_eq!("C", format!("{}", super::IPClass::C))
    }

    #[test]
    fn class_d() {
        assert_eq!("D", format!("{}", super::IPClass::D))
    }

    #[test]
    fn class_e() {
        assert_eq!("E", format!("{}", super::IPClass::E))
    }
}

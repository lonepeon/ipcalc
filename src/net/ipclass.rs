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

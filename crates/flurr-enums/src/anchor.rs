bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Anchor: u8 {
        const TOP = 0x01;
        const RIGHT = 0x02;
        const BOTTOM = 0x04;
        const LEFT = 0x08;
    }
}

impl From<Anchor> for u8 {
    fn from(value: Anchor) -> Self {
        value.bits()
    }
}

impl TryFrom<u8> for Anchor {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match Self::from_bits(value) {
            Some(anchor) => Ok(anchor),
            None => Err(()),
        }
    }
}

impl std::str::FromStr for Anchor {
    type Err = bitflags::parser::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        bitflags::parser::from_str(s)
    }
}

impl std::fmt::Display for Anchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        bitflags::parser::to_writer(self, f)
    }
}

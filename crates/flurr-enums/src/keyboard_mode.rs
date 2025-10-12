#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(
        strum::EnumString,
        strum::FromRepr,
        strum::AsRefStr,
        strum::IntoStaticStr
    )
)]
#[cfg_attr(feature = "strum", strum(ascii_case_insensitive))]
pub enum KeyboardMode {
    None,
    Exclusive,
    OnDemand,
}

impl std::fmt::Display for KeyboardMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) // Same as Debug
    }
}

impl TryFrom<u8> for KeyboardMode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(KeyboardMode::None),
            1u8 => Ok(KeyboardMode::Exclusive),
            2u8 => Ok(KeyboardMode::OnDemand),
            _ => Err(()),
        }
    }
}

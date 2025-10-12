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

impl TryFrom<i32> for KeyboardMode {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == KeyboardMode::None as i32 => Ok(KeyboardMode::None),
            x if x == KeyboardMode::Exclusive as i32 => Ok(KeyboardMode::Exclusive),
            x if x == KeyboardMode::OnDemand as i32 => Ok(KeyboardMode::OnDemand),
            _ => Err(()),
        }
    }
}

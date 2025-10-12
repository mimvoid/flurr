#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
pub enum Layer {
    Background,
    Bottom,
    Top,
    Overlay,
}

impl std::fmt::Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) // Same as Debug
    }
}

impl TryFrom<u8> for Layer {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0u8 => Ok(Layer::Background),
            1u8 => Ok(Layer::Bottom),
            2u8 => Ok(Layer::Top),
            3u8 => Ok(Layer::Overlay),
            _ => Err(()),
        }
    }
}

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

impl TryFrom<i32> for Layer {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == Layer::Background as i32 => Ok(Layer::Background),
            x if x == Layer::Bottom as i32 => Ok(Layer::Bottom),
            x if x == Layer::Top as i32 => Ok(Layer::Top),
            x if x == Layer::Overlay as i32 => Ok(Layer::Overlay),
            _ => Err(()),
        }
    }
}

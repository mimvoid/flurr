#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Layer {
    Background,
    Bottom,
    Top,
    Overlay,
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


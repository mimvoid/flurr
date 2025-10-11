bitflags::bitflags! {
    pub struct Anchor: u8 {
        const TOP = 0x01;
        const RIGHT = 0x02;
        const BOTTOM = 0x04;
        const LEFT = 0x08;
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Layer {
    Background,
    Bottom,
    Top,
    Overlay,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum KeyboardMode {
    None,
    Exclusive,
    OnDemand,
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

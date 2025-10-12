#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum KeyboardMode {
    None,
    Exclusive,
    OnDemand,
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

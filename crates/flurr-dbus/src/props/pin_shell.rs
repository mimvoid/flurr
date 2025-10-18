use super::{BlockingProperties, PropertyError};

#[derive(Debug, Default)]
pub struct PinShellProps {
    pub unlocked: bool,
}

impl PinShellProps {
    pub fn get_blocking(proxy: &impl BlockingProperties) -> Result<Self, PropertyError> {
        let props = proxy.get_all("io.flurr.PinShell")?;
        Self::from_prop_map(&props)
    }

    pub fn from_prop_map(props: &dbus::arg::PropMap) -> Result<Self, PropertyError> {
        Ok(PinShellProps {
            unlocked: super::get_cast!("io.flurr.PinShell", props, "Unlocked", bool),
        })
    }
}

impl TryFrom<dbus::arg::PropMap> for PinShellProps {
    type Error = PropertyError;
    fn try_from(value: dbus::arg::PropMap) -> Result<Self, Self::Error> {
        Self::from_prop_map(&value)
    }
}

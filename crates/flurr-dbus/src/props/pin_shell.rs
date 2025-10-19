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
            unlocked: super::parse_cast(props, "Unlocked", "io.flurr.PinShell")?,
        })
    }
}

super::try_from_prop_map!(PinShellProps);

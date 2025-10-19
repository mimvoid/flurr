use super::{BlockingProperties, PropertyError, parse_string};

#[derive(Debug, Default)]
pub struct ShellProps {
    pub namespace: String,
    pub layer: u8,
    pub keyboard_mode: u8,
    pub anchor: u8,
    pub exclusion: i32,
    pub auto_exclusion: bool,
    pub margin_top: i32,
    pub margin_right: i32,
    pub margin_bottom: i32,
    pub margin_left: i32,
}

impl ShellProps {
    pub fn get_blocking(proxy: &impl BlockingProperties) -> Result<Self, PropertyError> {
        let props = proxy.get_all("io.flurr.Shell")?;
        Self::from_prop_map(&props)
    }

    pub fn from_prop_map(props: &dbus::arg::PropMap) -> Result<Self, PropertyError> {
        macro_rules! parse_cast {
            ($prop_name: expr) => {
                super::parse_cast(props, $prop_name, "io.flurr.Shell")
            };
        }

        Ok(ShellProps {
            namespace: parse_string(props, "Namespace", "io.flurr.Shell")?,
            layer: parse_cast!("Layer")?,
            keyboard_mode: parse_cast!("KeyboardMode")?,
            anchor: parse_cast!("Anchor")?,
            exclusion: parse_cast!("Exclusion")?,
            auto_exclusion: parse_cast!("AutoExclusion")?,
            margin_top: parse_cast!("MarginTop")?,
            margin_right: parse_cast!("MarginRight")?,
            margin_bottom: parse_cast!("MarginBottom")?,
            margin_left: parse_cast!("MarginLeft")?,
        })
    }
}

super::try_from_prop_map!(ShellProps);

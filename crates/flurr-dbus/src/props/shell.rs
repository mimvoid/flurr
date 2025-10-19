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
        macro_rules! get_cast {
            ($prop_name: expr, $type: ty) => {
                super::get_cast!("io.flurr.Shell", props, $prop_name, $type)
            };
        }

        Ok(ShellProps {
            namespace: parse_string!("io.flurr.Shell", props, "Namespace"),
            layer: get_cast!("Layer", u8),
            keyboard_mode: get_cast!("KeyboardMode", u8),
            anchor: get_cast!("Anchor", u8),
            exclusion: get_cast!("Exclusion", i32),
            auto_exclusion: get_cast!("AutoExclusion", bool),
            margin_top: get_cast!("MarginTop", i32),
            margin_right: get_cast!("MarginRight", i32),
            margin_bottom: get_cast!("MarginBottom", i32),
            margin_left: get_cast!("MarginLeft", i32),
        })
    }
}

impl TryFrom<dbus::arg::PropMap> for ShellProps {
    type Error = PropertyError;
    fn try_from(value: dbus::arg::PropMap) -> Result<Self, Self::Error> {
        Self::from_prop_map(&value)
    }
}

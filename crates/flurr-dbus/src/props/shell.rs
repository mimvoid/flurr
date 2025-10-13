use super::PropertyError;
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties as BlockingProperties;

#[derive(Debug, Default)]
pub struct ShellProps {
    pub namespace: String,
    pub layer: u8,
    pub keyboard_mode: u8,
    pub anchor: u8,
    pub zindex: i32,
    pub auto_exclusive_zone: bool,
    pub margin_top: i32,
    pub margin_right: i32,
    pub margin_bottom: i32,
    pub margin_left: i32,
}

impl ShellProps {
    pub fn get_blocking(proxy: &impl BlockingProperties) -> Result<Self, PropertyError> {
        let props = proxy.get_all("io.flurr.Shell")?;
        ShellProps::from_prop_map(&props)
    }

    pub fn from_prop_map(props: &dbus::arg::PropMap) -> Result<Self, PropertyError> {
        macro_rules! parse {
            ($prop_name: expr, $and_then: expr) => {
                props
                    .get($prop_name)
                    .and_then($and_then)
                    .ok_or_else(|| PropertyError::Parse {
                        interface: "io.flurr.Shell".to_owned(),
                        name: $prop_name.to_owned(),
                    })?
            };
        }
        macro_rules! get_cast {
            ($prop_name: expr, $type: ty) => {
                parse!($prop_name, |value| value.0.as_any().downcast_ref::<$type>()).to_owned()
            };
        }

        Ok(ShellProps {
            namespace: parse!("Namespace", |n| n.0.as_str()).to_owned(),
            layer: get_cast!("Layer", u8),
            keyboard_mode: get_cast!("KeyboardMode", u8),
            anchor: get_cast!("Anchor", u8),
            zindex: get_cast!("ZIndex", i32),
            auto_exclusive_zone: get_cast!("AutoExclusiveZone", bool),
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

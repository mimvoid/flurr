mod pin_shell;
mod shell;
mod window;

pub use pin_shell::PinShellProps;
pub use shell::ShellProps;
pub use window::WindowProps;

use dbus::arg::PropMap;
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties as BlockingProperties;

#[derive(Debug)]
pub enum PropertyError {
    Parse { interface: String, name: String },
    DBus(dbus::Error),
}

impl std::error::Error for PropertyError {}

impl std::fmt::Display for PropertyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PropertyError::Parse { interface, name } => {
                write!(f, "Failed to parse {} property \"{}\"", interface, name)
            }
            PropertyError::DBus(dbus_err) => write!(f, "{}", dbus_err),
        }
    }
}

impl From<dbus::Error> for PropertyError {
    fn from(value: dbus::Error) -> Self {
        PropertyError::DBus(value)
    }
}

fn parse<T, F>(
    props: &PropMap,
    prop_name: &str,
    iface: &str,
    and_then: F,
) -> Result<T, PropertyError>
where
    F: Fn(&dbus::arg::Variant<Box<dyn dbus::arg::RefArg>>) -> Option<T>,
{
    props
        .get(prop_name)
        .and_then(and_then)
        .ok_or_else(|| PropertyError::Parse {
            interface: iface.to_owned(),
            name: prop_name.to_owned(),
        })
}

fn parse_string(props: &PropMap, prop_name: &str, iface: &str) -> Result<String, PropertyError> {
    parse(props, prop_name, iface, |s| s.0.as_str().map(str::to_owned))
}

fn parse_cast<T>(props: &PropMap, prop_name: &str, iface: &str) -> Result<T, PropertyError>
where
    T: Clone + 'static,
{
    parse(props, prop_name, iface, |value| {
        value.0.as_any().downcast_ref::<T>().map(T::to_owned)
    })
}

macro_rules! try_from_prop_map {
    ($struct: ident) => {
        impl TryFrom<dbus::arg::PropMap> for $struct {
            type Error = PropertyError;
            fn try_from(value: dbus::arg::PropMap) -> Result<Self, Self::Error> {
                Self::from_prop_map(&value)
            }
        }
    };
}
pub(self) use try_from_prop_map;

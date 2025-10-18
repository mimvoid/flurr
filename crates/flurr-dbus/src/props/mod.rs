mod pin_shell;
mod shell;
mod window;

pub use pin_shell::PinShellProps;
pub use shell::ShellProps;
pub use window::WindowProps;

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

macro_rules! parse {
    ($iface: expr, $props: expr, $prop_name: expr, $and_then: expr) => {
        $props
            .get($prop_name)
            .and_then($and_then)
            .ok_or_else(|| PropertyError::Parse {
                interface: $iface.to_owned(),
                name: $prop_name.to_owned(),
            })?
    };
}
pub(self) use parse;

macro_rules! parse_string {
    ($iface: expr, $props: expr, $prop_name: expr) => {
        crate::props::parse!($iface, $props, $prop_name, |s| s.0.as_str()).to_owned()
    };
}
pub(self) use parse_string;

macro_rules! get_cast {
    ($iface: expr, $props: expr, $prop_name: expr, $type: ty) => {
        crate::props::parse!($iface, $props, $prop_name, |value| value
            .0
            .as_any()
            .downcast_ref::<$type>())
        .to_owned()
    };
}
pub(self) use get_cast;

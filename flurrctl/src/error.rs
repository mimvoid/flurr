use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ServiceUnknown(String),
    WindowError {
        name: Option<String>,
        path: dbus::Path<'static>,
        message: Option<String>,
    },
    /// Wrapper for dbus-rs errors
    DBus(dbus::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DBusError {
    Unknown = 0,
    ServiceUnknown = 1,
    UnknownMethod = 2,
}
impl From<&str> for DBusError {
    fn from(value: &str) -> Self {
        match value {
            "org.freedesktop.DBus.Error.ServiceUnknown" => DBusError::ServiceUnknown,
            "org.freedesktop.DBus.Error.UnknownMethod" => DBusError::UnknownMethod,
            _ => DBusError::Unknown,
        }
    }
}
impl From<&dbus::Error> for DBusError {
    fn from(value: &dbus::Error) -> Self {
        match value.name() {
            Some(name) => name.into(),
            None => DBusError::Unknown,
        }
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ServiceUnknown(instance) => {
                write!(f, r#"Couldn't find the instance "{}""#, instance)
            }
            Error::WindowError {
                name,
                path,
                message,
            } => write!(
                f,
                "Failed to call window{}{}{}",
                name.as_ref()
                    .map(|n| [" \"", n, "\""].concat())
                    .as_deref()
                    .unwrap_or_default(),
                path.rsplit_once('/')
                    .map(|p| format!(" (id {})", p.1))
                    .as_deref()
                    .unwrap_or_default(),
                message
                    .as_ref()
                    .map(|m| [": ", m].concat())
                    .as_deref()
                    .unwrap_or_default()
            ),
            Error::DBus(dbus_err) => match dbus_err.name() {
                Some(name) => write!(f, "{}: {}", name, dbus_err),
                None => write!(f, "{}", dbus_err),
            },
        }
    }
}

impl From<dbus::Error> for Error {
    fn from(value: dbus::Error) -> Self {
        Error::DBus(value)
    }
}

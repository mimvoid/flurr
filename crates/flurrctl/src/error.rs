pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
/// The main error used throughout flurrctl to keep track of relevant information and print them in
/// a user-friendly way
pub enum Error {
    /// For when the DBus service cannot be found
    #[error("Couldn't find the instance \"{0}\"")]
    ServiceUnknown(String),

    /// For window method errors with a known DBus object path
    #[error("Failed to call window{}{}{}", .name.as_ref()
                    .map_or_else(String::default, |n| format!(" \"{n}\"")),
                .path.rsplit_once('/')
                    .map_or_else(String::default, |p| format!(" (id {})", p.1)),
                .dbus_error.message()
                    .map_or_else(String::default, |m| format!(": {m}")))]
    WindowError {
        name: Option<String>,
        path: dbus::Path<'static>,
        dbus_error: dbus::Error,
    },

    /// Wrapper for dbus-rs errors
    #[error("{}{}", .0.name().map_or_else(String::default, |n| format!("{n}: ")), .0)]
    DBus(#[from] dbus::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Non-exhaustive standard DBus error types under the "org.freedesktop.DBus.Error" namespace
pub enum DBusError {
    /// For any unimplemented or unparseable error
    Unknown = 0,
    ServiceUnknown = 1,
    UnknownMethod = 2,
}

impl Error {
    pub fn parse_dbus_name(err: dbus::Error, instance: &str) -> Self {
        match err.name() {
            Some("org.freedesktop.DBus.Error.ServiceUnknown") => {
                Error::ServiceUnknown(instance.into())
            }
            _ => Error::DBus(err),
        }
    }
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

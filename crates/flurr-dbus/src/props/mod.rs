mod shell;
pub use shell::ShellProps;

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

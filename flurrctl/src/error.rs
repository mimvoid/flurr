use std::fmt;

#[derive(Debug)]
pub enum FlurrctlError {
    ServiceUnknown { instance: String },
}

impl std::error::Error for FlurrctlError {}

impl fmt::Display for FlurrctlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use FlurrctlError::*;
        match self {
            ServiceUnknown { instance } => {
                let inst = instance.as_str();
                write!(
                    f,
                    "Service io.flurr.{inst} was not found. Is the instance \"{inst}\" running?",
                )
            }
        }
    }
}

impl FlurrctlError {
    pub fn new(instance: String, dbus_error: &dbus::Error) -> Option<Self> {
        match dbus_error.name() {
            Some("org.freedesktop.DBus.Error.ServiceUnknown") => {
                Some(FlurrctlError::ServiceUnknown { instance })
            }
            _ => None,
        }
    }
}

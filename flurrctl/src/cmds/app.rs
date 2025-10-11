use dbus::blocking::Connection;
use flurr_dbus::Application;

use crate::error::{DBusError, Error};

pub fn quit(conn: &Connection, instance: &str) -> crate::Result<()> {
    let app = Application::new(&conn, instance);

    if let Err(err) = app.quit() {
        Err(match DBusError::from(&err) {
            DBusError::ServiceUnknown => Error::ServiceUnknown(instance.into()),
            _ => Error::DBus(err),
        })
    } else {
        log::info!("Successfully quit instance \"{}\"", instance);
        Ok(())
    }
}

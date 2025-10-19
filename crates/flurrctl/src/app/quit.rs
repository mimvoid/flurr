use dbus::blocking::Connection;
use flurr_dbus::Application;

use crate::error::Error;

pub fn quit(conn: &Connection, instances: &[String]) -> crate::Result<()> {
    for instance in instances {
        quit_instance(conn, instance.as_ref())?;
    }
    Ok(())
}

fn quit_instance(conn: &Connection, instance: &str) -> crate::Result<()> {
    let app = Application::new(&conn, instance);

    match app.quit() {
        Ok(()) => {
            log::info!("Successfully quit instance \"{instance}\"");
            Ok(())
        }
        Err(err) => Err(Error::parse_dbus_name(err, instance)),
    }
}

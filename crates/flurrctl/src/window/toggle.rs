use crate::{DBusError, Error};
use dbus::blocking::Connection;
use flurr_dbus::{Window, ProxyWrapper};

/// Helper function to execute a given method on a window, log relevant information,
/// and process any errors.
fn window_method<F>(
    conn: &Connection,
    instance: &str,
    window: &str,
    info_msg: &str,
    f: F,
) -> crate::Result<()>
where
    F: Fn(&Window) -> dbus::Result<()>,
{
    log::info!("Finding window");
    let window_path = super::get_window_path(&conn, instance, window)?;
    let window_proxy = Window::with_path(&conn, instance, window_path);

    log::info!("{info_msg}");

    f(&window_proxy).map_err(|err| match DBusError::from(&err) {
        DBusError::ServiceUnknown => Error::ServiceUnknown(instance.to_owned()),
        DBusError::UnknownMethod => Error::WindowError {
            name: window.to_string(),
            path: window_proxy.proxy().path.clone(),
            dbus_error: err,
        },
        _ => Error::DBus(err),
    })?;
    Ok(())
}

pub fn toggle_window(conn: &Connection, instance: &str, window: &str) -> crate::Result<()> {
    window_method(conn, instance, window, "Toggling window", |w| w.toggle())
}

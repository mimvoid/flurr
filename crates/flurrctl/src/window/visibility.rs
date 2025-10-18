use crate::{DBusError, Error};
use dbus::blocking::Connection;
use flurr_dbus::Window;

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
    let window_proxy = Window::with_path(&conn, instance, window_path.clone());

    log::info!("{info_msg}");

    f(&window_proxy).map_err(|err| match DBusError::from(&err) {
        DBusError::ServiceUnknown => Error::ServiceUnknown(instance.to_owned()),
        DBusError::UnknownMethod => Error::WindowError {
            name: window.to_string(),
            path: window_path,
            dbus_error: err,
        },
        _ => Error::DBus(err),
    })?;
    Ok(())
}

macro_rules! with_window_method {
    ($name: ident, $info_msg: expr, $method: expr) => {
        pub fn $name(conn: &Connection, instance: &str, window: &str) -> crate::Result<()> {
            window_method(conn, instance, window, $info_msg, $method)
        }
    };
}

with_window_method!(toggle_window, "Toggling window", |w| w.toggle());
with_window_method!(show_window, "Showing window", |w| w.set_visible(true));
with_window_method!(hide_window, "Hiding window", |w| w.set_visible(false));

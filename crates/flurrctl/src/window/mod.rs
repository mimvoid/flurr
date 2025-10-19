mod get;
mod set;
mod toggle;

pub use get::get_windows;
pub use set::set_window;
pub use toggle::toggle_window;

use dbus::blocking::Connection;

/// Get a window's DBus object path from a name or id.
///
/// If an id is given, the window may or may not exist.
fn get_window_path<'a>(
    conn: &'a Connection,
    instance: &str,
    window: &str,
) -> crate::Result<dbus::Path<'static>> {
    if let Ok(id) = window.parse::<u32>() {
        return Ok(flurr_dbus::make_window_path(instance, &id));
    }

    let app = flurr_dbus::Application::new(conn, instance);
    app.get_window_path(window)
        .map_err(|err| crate::Error::parse_dbus_name(err, instance))
}

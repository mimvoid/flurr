mod list;
mod visibility;

pub use list::print_windows;
pub use visibility::{hide_window, show_window, toggle_window};

use dbus::blocking::Connection;

/// Get a window's DBus object path from a name or id.
///
/// If an id is given, the window may or may not exist.
fn get_window_path<'a>(
    conn: &'a Connection,
    instance: &str,
    opts: &crate::args::WindowCommand,
) -> crate::Result<dbus::Path<'static>> {
    if let Some(id) = opts.id {
        return Ok(flurr_dbus::make_window_path(instance, &id));
    }

    if let Some(name) = opts.name.as_deref() {
        let app = flurr_dbus::Application::new(conn, instance);
        return app
            .get_window_path(name)
            .map_err(|err| crate::Error::parse_dbus_name(err, instance));
    }

    unreachable!("Clap ensures either a name or an id is provided")
}

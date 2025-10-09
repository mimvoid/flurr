use crate::args::WindowCommand;
use dbus::blocking::Connection;
use flurr_dbus::{Application, Window};

macro_rules! with_window_methods {
    ($name: ident, $method: expr, $info_msg: expr) => {
        pub fn $name(conn: &Connection, instance: &str, opts: &WindowCommand) -> dbus::Result<()> {
            log::info!("Finding window");
            let window_path = get_window_path(&conn, instance, opts)?;
            let window = Window::with_path(&conn, instance, window_path);

            log::info!($info_msg);
            $method(&window)?;

            Ok(())
        }
    };
}

with_window_methods!(toggle_window, Window::toggle, "Toggling window");
with_window_methods!(
    show_window,
    (|proxy| Window::set_visible(proxy, true)),
    "Showing window"
);
with_window_methods!(
    hide_window,
    (|proxy| Window::set_visible(proxy, false)),
    "Hiding window"
);

fn get_window_path<'a>(
    conn: &'a Connection,
    instance: &str,
    opts: &WindowCommand,
) -> dbus::Result<dbus::Path<'static>> {
    if let Some(name) = opts.name.as_deref() {
        Application::new(conn, instance).get_window_path(name)
    } else if let Some(id) = opts.id {
        Ok(flurr_dbus::make_window_path(instance, &id))
    } else {
        unreachable!()
    }
}

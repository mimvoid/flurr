use crate::{TIMEOUT, args::WindowCommand};
use dbus::blocking::Connection;
use flurr_dbus::{Application, Window, application_proxy};

macro_rules! with_window_methods {
    ($name: ident, $method: expr) => {
        pub fn $name(instance: &str, opts: &WindowCommand) -> dbus::Result<()> {
            let conn = Connection::new_session()?;
            let window_path = get_window_path(&conn, instance, opts)?;

            let window = conn.with_proxy(format!("io.flurr.{instance}"), window_path, TIMEOUT);
            $method(&window)?;

            Ok(())
        }
    };
}

with_window_methods!(toggle_window, Window::toggle);
with_window_methods!(show_window, (|proxy| Window::set_visible(proxy, true)));
with_window_methods!(hide_window, (|proxy| Window::set_visible(proxy, false)));

fn get_window_path<'a>(
    conn: &'a Connection,
    instance: &str,
    opts: &WindowCommand,
) -> dbus::Result<dbus::Path<'static>> {
    if let Some(name) = opts.name.as_deref() {
        application_proxy(conn, instance).get_window_path(name)
    } else if let Some(id) = opts.id {
        Ok(dbus::Path::from(format!(
            "/io/flurr/{instance}/window/{id}"
        )))
    } else {
        unreachable!()
    }
}

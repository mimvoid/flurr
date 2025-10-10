use crate::{DBusError, Error, args::WindowCommand};
use dbus::blocking::Connection;
use flurr_dbus::{Application, Window};

fn window_method<F>(
    conn: &Connection,
    instance: &str,
    opts: &WindowCommand,
    info_msg: &str,
    f: F,
) -> crate::Result<()>
where
    F: Fn(&Window) -> dbus::Result<()>,
{
    log::info!("Finding window");
    let window_path = get_window_path(&conn, instance, opts)?;
    let window = Window::with_path(&conn, instance, window_path.clone());

    log::info!("{}", info_msg);

    if let Err(err) = f(&window) {
        Err(match DBusError::from(&err) {
            DBusError::ServiceUnknown => Error::ServiceUnknown(instance.to_owned()),
            DBusError::UnknownMethod => Error::WindowError {
                name: opts.name.clone(),
                path: window_path,
                message: err.message().map(String::from),
            },
            _ => Error::DBus(err),
        })
    } else {
        Ok(())
    }
}

macro_rules! with_window_method {
    ($name: ident, $info_msg: expr, $method: expr) => {
        pub fn $name(conn: &Connection, instance: &str, opts: &WindowCommand) -> crate::Result<()> {
            window_method(conn, instance, opts, $info_msg, $method)
        }
    };
}

with_window_method!(toggle_window, "Toggling window", |w| w.toggle());
with_window_method!(show_window, "Showing window", |w| w.set_visible(true));
with_window_method!(hide_window, "Hiding window", |w| w.set_visible(false));

fn get_window_path<'a>(
    conn: &'a Connection,
    instance: &str,
    opts: &WindowCommand,
) -> crate::Result<dbus::Path<'static>> {
    if let Some(id) = opts.id {
        return Ok(flurr_dbus::make_window_path(instance, &id));
    }

    if let Some(name) = opts.name.as_deref() {
        let app = Application::new(conn, instance);

        return match app.get_window_path(name) {
            Ok(path) => Ok(path),
            Err(e) => Err(match DBusError::from(&e) {
                DBusError::ServiceUnknown => Error::ServiceUnknown(instance.into()),
                _ => Error::DBus(e),
            }),
        };
    }

    unreachable!()
}

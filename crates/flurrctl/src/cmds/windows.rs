use dbus::blocking::Connection;
use flurr_dbus::{Application, PinShell, Shell, ShellProps, Window};
use std::io::{Write, stdout};

use crate::Error;

pub fn print_windows(conn: &Connection, instance: &str) -> crate::Result<()> {
    let paths = list_window_paths(conn, instance)?;
    let mut lock = stdout().lock();

    for path in paths {
        let window = Window::with_path(conn, instance, path.clone());

        let _ = writeln!(
            lock,
            "{}:",
            window.name().unwrap_or_else(|_| String::from("Window"))
        );

        if let Some(id) = path.rsplit_once('/') {
            let _ = writeln!(lock, "  id: {}", id.1);
        }

        if let Ok(props) = Shell::get_all(&window) {
            let _ = writeln!(lock, "{}", display_shell_props(&props));
        }
        if let Ok(unlocked) = PinShell::unlocked(&window) {
            let _ = writeln!(lock, "  unlocked: {}", unlocked);
        }

        let _ = writeln!(lock, "");
    }

    Ok(())
}

fn display_shell_props(props: &ShellProps) -> String {
    format!(
        "  namespace: \"{}\"\n  layer: {}\n  keyboard_mode: {}\n  anchor: {}\n  auto_exclusive_zone: {}\n  z_index: {}\n  margin_top: {}\n  margin_right: {}\n  margin_bottom: {}\n  margin_left: {}",
        props.namespace,
        props.layer,
        props.keyboard_mode,
        props.anchor,
        props.auto_exclusive_zone,
        props.zindex,
        props.margin_top,
        props.margin_right,
        props.margin_left,
        props.margin_bottom
    )
}

fn list_window_paths(conn: &Connection, instance: &str) -> crate::Result<Vec<dbus::Path<'static>>> {
    let app = Application::new(conn, instance);
    match app.list_window_paths() {
        Ok(paths) => Ok(paths),
        Err(err) => Err(Error::parse_dbus_name(err, instance)),
    }
}

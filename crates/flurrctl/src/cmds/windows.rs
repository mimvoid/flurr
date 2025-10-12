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
            if let Some(prop_str) = display_shell_props(&props) {
                let _ = writeln!(lock, "{prop_str}");
            }
        }
        if let Ok(unlocked) = PinShell::unlocked(&window) {
            let _ = writeln!(lock, "  unlocked: {unlocked}");
        }

        let _ = writeln!(lock, "");
    }

    Ok(())
}

fn display_shell_props(props: &ShellProps) -> Option<String> {
    let Ok(layer) = flurr_enums::Layer::try_from(props.layer as u8) else {
        log::warn!("Couldn't parse layer value: {}", props.layer);
        return None;
    };

    let Ok(keyboard_mode) = flurr_enums::KeyboardMode::try_from(props.keyboard_mode as u8) else {
        log::warn!("Couldn't parse keyboard_mode value: {}", props.keyboard_mode);
        return None;
    };

    let Ok(anchor) = flurr_enums::Anchor::try_from(props.anchor) else {
        log::warn!("Couldn't parse anchor value: {}", props.anchor);
        return None;
    };

    Some(format!(
        "  namespace: \"{}\"\n  layer: {}\n  keyboard_mode: {}\n  anchor: {}\n  auto_exclusive_zone: {}\n  z_index: {}\n  margin_top: {}\n  margin_right: {}\n  margin_bottom: {}\n  margin_left: {}",
        props.namespace,
        layer,
        keyboard_mode,
        anchor,
        props.auto_exclusive_zone,
        props.zindex,
        props.margin_top,
        props.margin_right,
        props.margin_left,
        props.margin_bottom
    ))
}

fn list_window_paths(conn: &Connection, instance: &str) -> crate::Result<Vec<dbus::Path<'static>>> {
    let app = Application::new(conn, instance);
    match app.list_window_paths() {
        Ok(paths) => Ok(paths),
        Err(err) => Err(Error::parse_dbus_name(err, instance)),
    }
}

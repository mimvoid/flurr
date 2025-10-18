use dbus::blocking::Connection;
use flurr_dbus::{Application, Window, props};
use std::io::{Write, stdout};

use crate::{Error, args::GetCommand};

pub fn get_windows(conn: &Connection, instance: &str, opts: &GetCommand) -> crate::Result<()> {
    let paths = if opts.windows.is_empty() {
        get_paths_all(conn, instance)?
    } else {
        get_paths(conn, instance, &opts.windows)?
    };

    let id_props = paths.iter().map(|path| {
        let window = Window::with_path(conn, instance, path.clone());
        let props = props::WindowProps::get_blocking(&window.proxy);
        props.map(|p| (path.rsplit_once('/').map(|s| s.1), p))
    });

    let mut lock = stdout().lock();

    for res in id_props {
        let (id_opt, props) = res?;

        let _ = writeln!(lock, "{}:", props.name);
        if let Some(id) = id_opt {
            let _ = writeln!(lock, "  id: {id}");
        }

        let _ = writeln!(lock, "  visible: {}", props.visible);

        if let Some(shell) = props.shell() {
            let _ = writeln!(lock, "{}", display_shell_props(shell, opts.raw));
        }
        if let Some(pin_shell) = props.pin_shell() {
            let _ = writeln!(lock, "  unlocked: {}", pin_shell.unlocked);
        }
    }

    Ok(())
}

fn display_shell_props(props: &props::ShellProps, raw: bool) -> String {
    let enum_flags = if raw {
        format!(
            "  layer: {}\n  keyboard_mode: {}\n  anchor: {}",
            props.layer, props.keyboard_mode, props.anchor
        )
    } else {
        shell_props_pretty(props.layer, props.keyboard_mode, props.anchor)
    };

    format!(
        "  namespace: \"{}\"\n{enum_flags}\n  auto_exclusive_zone: {}\n  z_index: {}\n  margin_top: {}\n  margin_right: {}\n  margin_bottom: {}\n  margin_left: {}",
        props.namespace,
        props.auto_exclusive_zone,
        props.zindex,
        props.margin_top,
        props.margin_right,
        props.margin_bottom,
        props.margin_left
    )
}

fn shell_props_pretty(layer_prop: u8, keyboard_mode_prop: u8, anchor_prop: u8) -> String {
    let layer = match flurr_enums::Layer::try_from(layer_prop) {
        Ok(l) => l.to_string(),
        _ => {
            log::warn!("Couldn't parse layer: {layer_prop}");
            layer_prop.to_string()
        }
    };

    let keyboard_mode = match flurr_enums::KeyboardMode::try_from(keyboard_mode_prop) {
        Ok(k) => k.to_string(),
        _ => {
            log::warn!("Couldn't parse keyboard_mode: {keyboard_mode_prop}");
            keyboard_mode_prop.to_string()
        }
    };

    let anchor = flurr_enums::Anchor::try_from(anchor_prop).unwrap_or_else(|_| {
        log::warn!("Unsetting unknown bits in anchor: {anchor_prop}");
        flurr_enums::Anchor::from_bits_truncate(anchor_prop)
    });

    format!("  layer: {layer}\n  keyboard_mode: {keyboard_mode}\n  anchor: {anchor}")
}

fn get_paths(
    conn: &Connection,
    instance: &str,
    windows: &[String],
) -> crate::Result<Vec<dbus::Path<'static>>> {
    let app = Application::new(conn, instance);
    let mut paths = Vec::<dbus::Path<'static>>::new();

    for window in windows {
        if let Ok(id) = window.parse::<u32>() {
            paths.push(flurr_dbus::make_window_path(instance, &id));
        } else {
            let path = app
                .get_window_path(window)
                .map_err(|err| crate::Error::parse_dbus_name(err, instance))?;
            paths.push(path);
        }
    }

    Ok(paths)
}

fn get_paths_all(conn: &Connection, instance: &str) -> crate::Result<Vec<dbus::Path<'static>>> {
    let app = Application::new(conn, instance);
    app.list_window_paths()
        .map_err(|err| Error::parse_dbus_name(err, instance))
}

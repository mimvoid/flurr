use dbus::blocking::Connection;
use flurr_dbus::{Application, Window, props, props::ShellProps};
use std::io::{Write, stdout};

use crate::{Error, args::GetCommand};

pub fn get_windows(conn: &Connection, opts: &GetCommand) -> crate::Result<()> {
    let instance = opts.instance.as_ref();
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

        writeln!(lock, "{}:", props.name)?;
        if let Some(id) = id_opt {
            writeln!(lock, "  id: {id}")?;
        }

        writeln!(lock, "  visible: {}", props.visible)?;

        if let Some(shell) = props.shell() {
            display_shell_props(&mut lock, shell, opts.raw)?;
        }
        if let Some(pin_shell) = props.pin_shell() {
            writeln!(lock, "  unlocked: {}", pin_shell.unlocked)?;
        }
    }

    Ok(())
}

fn display_shell_props(buf: &mut impl Write, props: &ShellProps, raw: bool) -> crate::Result<()> {
    writeln!(buf, "  namespace: \"{}\"", props.namespace)?;

    if raw {
        writeln!(
            buf,
            "  layer: {}\n  keyboard_mode: {}\n  anchor: {}",
            props.layer, props.keyboard_mode, props.anchor
        )?;
    } else {
        pretty_enums(buf, props.layer, props.keyboard_mode, props.anchor)?;
    };

    writeln!(
        buf,
        "  auto_exclusive_zone: {}\n  z_index: {}\n  margin_top: {}\n  margin_right: {}\n  margin_bottom: {}\n  margin_left: {}",
        props.auto_exclusive_zone,
        props.zindex,
        props.margin_top,
        props.margin_right,
        props.margin_bottom,
        props.margin_left
    )?;

    Ok(())
}

fn pretty_enums(buf: &mut impl Write, layer: u8, keyboard: u8, anchor: u8) -> crate::Result<()> {
    match flurr_enums::Layer::try_from(layer) {
        Ok(l) => writeln!(buf, "  layer: {l}"),
        _ => {
            log::warn!("Couldn't parse layer: {layer}");
            writeln!(buf, "  layer: {layer}")
        }
    }?;

    match flurr_enums::KeyboardMode::try_from(keyboard) {
        Ok(k) => writeln!(buf, "  keyboard_mode: {k}"),
        _ => {
            log::warn!("Couldn't parse keyboard_mode: {keyboard}");
            writeln!(buf, "  keyboard_mode: {keyboard}")
        }
    }?;

    let anchor = flurr_enums::Anchor::try_from(anchor).unwrap_or_else(|_| {
        log::warn!("Unsetting unknown bits in anchor: {anchor}");
        flurr_enums::Anchor::from_bits_truncate(anchor)
    });
    writeln!(buf, "  anchor: {anchor}")?;

    Ok(())
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

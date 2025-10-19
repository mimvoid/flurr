use dbus::blocking::Connection;
use flurr_dbus::{Application, ProxyWrapper, Window, props, props::ShellProps};
use heck::ToSnakeCase;
use std::io::{Write, stdout};

use crate::window::property_flags::PropertyFlags as Pf;
use crate::{Error, args::GetCommand};

macro_rules! writeln_if {
    ($flags: expr, $flag: expr, $buf: expr, $value: expr) => {
        if $flags.contains($flag) {
            writeln!(
                $buf,
                "  {}: {}",
                format!("{}", $flag).to_snake_case(),
                $value
            )?;
        }
    };
}

pub fn get_windows(conn: &Connection, opts: &GetCommand) -> crate::Result<()> {
    let instance = opts.instance.as_ref();
    let paths = if opts.windows.is_empty() {
        get_paths_all(conn, instance)?
    } else {
        get_paths(conn, instance, &opts.windows)?
    };

    let flags = match opts.properties.as_ref() {
        Some(prop_names) => Pf::parse_names(prop_names),
        None => Pf::all(),
    };

    let id_props = paths.iter().map(|path| {
        let win = Window::with_path(conn, instance, path);
        let props = props::WindowProps::get_blocking(win.proxy());
        props.map(|p| (win.proxy().path.rsplit_once('/').map(|s| s.1.to_owned()), p))
    });

    let mut lock = stdout().lock();

    for res in id_props {
        let (id_opt, props) = res.map_err(|err| match err {
            flurr_dbus::props::PropertyError::DBus(e) => Error::parse_dbus_name(e, instance),
            _ => err.into(),
        })?;

        // Buffer to write properties keys and values
        let mut buf = Vec::new();

        writeln_if!(flags, Pf::VISIBLE, buf, props.visible);

        if let Some(shell) = props.shell() {
            display_shell_props(&mut buf, shell, &flags, opts.raw)?;
        }
        if flags.contains(Pf::UNLOCKED) {
            if let Some(pin) = props.pin_shell() {
                writeln!(buf, "  unlocked: {}", pin.unlocked)?;
            }
        }

        if !buf.is_empty() {
            writeln!(lock, "{}:", props.name)?;
            if let Some(id) = id_opt {
                writeln!(lock, "  id: {id}")?;
            }
            lock.write_all(&buf)?;
        }
    }

    Ok(())
}

fn display_shell_props(
    buf: &mut impl Write,
    props: &ShellProps,
    flags: &Pf,
    raw: bool,
) -> crate::Result<()> {
    writeln_if!(flags, Pf::NAMESPACE, buf, props.namespace);

    if raw {
        writeln_if!(flags, Pf::LAYER, buf, props.layer);
        writeln_if!(flags, Pf::KEYBOARD_MODE, buf, props.keyboard_mode);
        writeln_if!(flags, Pf::ANCHOR, buf, props.anchor);
    } else {
        pretty_enums(buf, flags, props)?;
    };

    writeln_if!(flags, Pf::AUTO_EXCLUSION, buf, props.auto_exclusion);
    writeln_if!(flags, Pf::EXCLUSION, buf, props.exclusion);
    writeln_if!(flags, Pf::MARGIN_TOP, buf, props.margin_top);
    writeln_if!(flags, Pf::MARGIN_RIGHT, buf, props.margin_right);
    writeln_if!(flags, Pf::MARGIN_BOTTOM, buf, props.margin_bottom);
    writeln_if!(flags, Pf::MARGIN_LEFT, buf, props.margin_left);

    Ok(())
}

fn pretty_enums(buf: &mut impl Write, flags: &Pf, props: &ShellProps) -> crate::Result<()> {
    if flags.contains(Pf::LAYER) {
        match flurr_enums::Layer::try_from(props.layer) {
            Ok(layer) => writeln!(buf, "  layer: {layer}"),
            _ => {
                log::warn!("Couldn't parse layer: {}", props.layer);
                writeln!(buf, "  layer: {}", props.layer)
            }
        }?;
    }

    if flags.contains(Pf::KEYBOARD_MODE) {
        match flurr_enums::KeyboardMode::try_from(props.keyboard_mode) {
            Ok(k) => writeln!(buf, "  keyboard_mode: {k}"),
            _ => {
                log::warn!("Couldn't parse keyboard_mode: {}", props.keyboard_mode);
                writeln!(buf, "  keyboard_mode: {}", props.keyboard_mode)
            }
        }?;
    }

    if flags.contains(Pf::ANCHOR) {
        let anchor = flurr_enums::Anchor::try_from(props.anchor).unwrap_or_else(|_| {
            log::warn!("Unsetting unknown bits in anchor: {}", props.anchor);
            flurr_enums::Anchor::from_bits_truncate(props.anchor)
        });
        writeln!(buf, "  anchor: {anchor}")?;
    }

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
                .map_err(|err| Error::parse_dbus_name(err, instance))?;
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

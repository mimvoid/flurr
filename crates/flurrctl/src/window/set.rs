use dbus::blocking::Connection;
use flurr_dbus::{PinShell, Shell, Window};
use std::str::FromStr;

use crate::args::SetCommand;

pub fn set_window(conn: &Connection, opts: &SetCommand) -> crate::Result<()> {
    let instance = opts.instance.as_ref();
    let path = super::get_window_path(conn, instance, opts.window.as_str())?;
    let window = Window::with_path(conn, instance, path);

    if let Some(visible) = opts.visible {
        window.set_visible(visible)?;
    }

    if let Some(layer) = opts.layer.as_ref() {
        if let Ok(layer_raw) = layer.parse::<u8>() {
            window.set_layer(layer_raw)?;
        } else if let Ok(layer_enum) = flurr_enums::Layer::from_str(layer) {
            window.set_layer(layer_enum as u8)?;
        } else {
            log::warn!("Skipping unknown layer value: {layer}");
        }
    }

    if let Some(keyboard_mode) = opts.keyboard_mode.as_ref() {
        if let Ok(keymode_raw) = keyboard_mode.parse::<u8>() {
            window.set_keyboard_mode(keymode_raw)?;
        } else if let Ok(key_enum) = flurr_enums::KeyboardMode::from_str(keyboard_mode) {
            window.set_keyboard_mode(key_enum as u8)?;
        } else {
            log::warn!("Skipping unknown keyboard_mode value: {keyboard_mode}");
        }
    }

    if let Some(anchor) = opts.anchor.as_ref() {
        if let Ok(anchor_raw) = anchor.parse::<u8>() {
            window.set_anchor(anchor_raw)?;
        } else if let Ok(anchor_flags) = flurr_enums::Anchor::from_str(anchor) {
            window.set_anchor(anchor_flags.bits())?;
        } else {
            log::warn!("Skipping unknown anchor value: {anchor}");
        }
    }

    if let Some(zindex) = opts.exclusion {
        window.set_exclusion(zindex)?;
    }
    if let Some(auto_exclusion) = opts.auto_exclusion {
        window.set_auto_exclusion(auto_exclusion)?;
    }
    if let Some(mtop) = opts.margin_top {
        window.set_margin_top(mtop)?;
    }
    if let Some(mright) = opts.margin_right {
        window.set_margin_right(mright)?;
    }
    if let Some(mbot) = opts.margin_bottom {
        window.set_margin_bottom(mbot)?;
    }
    if let Some(mleft) = opts.margin_left {
        window.set_margin_left(mleft)?;
    }

    if let Some(unlocked) = opts.unlocked {
        window.set_unlocked(unlocked)?;
    }

    Ok(())
}

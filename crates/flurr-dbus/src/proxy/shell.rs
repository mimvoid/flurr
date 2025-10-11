use crate::Window;

#[derive(Debug, Default)]
pub struct ShellProps {
    pub namespace: String,
    pub layer: i8,
    pub keyboard_mode: i8,
    pub anchor: u8,
    pub zindex: i32,
    pub auto_exclusive_zone: bool,
    pub margin_top: i32,
    pub margin_right: i32,
    pub margin_bottom: i32,
    pub margin_left: i32,
}

pub trait Shell: hidden::Shell {
    fn get_all(&self) -> dbus::Result<ShellProps>;

    fn namespace(&self) -> dbus::Result<String>;

    fn layer(&self) -> dbus::Result<i32>;
    fn set_layer(&self, value: i32) -> dbus::Result<()>;

    fn keyboard_mode(&self) -> dbus::Result<i32>;
    fn set_keyboard_mode(&self, value: i32) -> dbus::Result<()>;

    fn anchor(&self) -> dbus::Result<u32>;
    fn set_anchor(&self, value: u32) -> dbus::Result<()>;

    fn zindex(&self) -> dbus::Result<i32>;
    fn set_zindex(&self, value: i32) -> dbus::Result<()>;

    fn auto_exclusive_zone(&self) -> dbus::Result<bool>;
    fn set_auto_exclusive_zone(&self, value: bool) -> dbus::Result<()>;

    fn margin_top(&self) -> dbus::Result<i32>;
    fn set_margin_top(&self, value: i32) -> dbus::Result<()>;

    fn margin_right(&self) -> dbus::Result<i32>;
    fn set_margin_right(&self, value: i32) -> dbus::Result<()>;

    fn margin_bottom(&self) -> dbus::Result<i32>;
    fn set_margin_bottom(&self, value: i32) -> dbus::Result<()>;

    fn margin_left(&self) -> dbus::Result<i32>;
    fn set_margin_left(&self, value: i32) -> dbus::Result<()>;
}

mod hidden {
    pub trait Shell {
        crate::proxy::dbus_default_trait!();
    }
}
impl<'a, 'b> hidden::Shell for Window<'a, 'b> {
    super::dbus_default_interface!("io.flurr.Shell");
}

impl<'a, 'b> Shell for Window<'a, 'b> {
    fn get_all(&self) -> dbus::Result<ShellProps> {
        use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
        let props = self.proxy.get_all("io.flurr.Shell")?;

        macro_rules! parse {
            ($var: ident, $prop_name: expr, $and_then: expr) => {
                let Some($var) = props.get($prop_name).and_then($and_then) else {
                    return Err(dbus::Error::new_failed(
                        r#"Couldn't parse "$prop_name\ for "io.flurr.Shell""#,
                    ));
                };
            };
        }

        parse!(namespace, "Namespace", |n| n.0.as_str());
        parse!(layer, "Layer", |l| l.0.as_i64());
        parse!(keymode, "KeyboardMode", |l| l.0.as_i64());
        parse!(anchor, "Anchor", |a| a.0.as_u64());
        parse!(zindex, "ZIndex", |z| z.0.as_i64());
        parse!(auto_ex, "AutoExclusiveZone", |a| a.0.as_i64());
        parse!(mtop, "MarginTop", |m| m.0.as_i64());
        parse!(mright, "MarginRight", |m| m.0.as_i64());
        parse!(mbottom, "MarginBottom", |m| m.0.as_i64());
        parse!(mleft, "MarginLeft", |m| m.0.as_i64());

        Ok(ShellProps {
            namespace: namespace.to_owned(),
            layer: layer as i8,
            keyboard_mode: keymode as i8,
            anchor: anchor as u8,
            zindex: zindex as i32,
            auto_exclusive_zone: auto_ex != 0,
            margin_top: mtop as i32,
            margin_right: mright as i32,
            margin_bottom: mbottom as i32,
            margin_left: mleft as i32,
        })
    }

    fn namespace(&self) -> dbus::Result<String> {
        hidden::Shell::get(self, "Namespace")
    }

    fn layer(&self) -> dbus::Result<i32> {
        hidden::Shell::get(self, "Layer")
    }
    fn set_layer(&self, value: i32) -> dbus::Result<()> {
        hidden::Shell::set(self, "Layer", value)
    }

    fn keyboard_mode(&self) -> dbus::Result<i32> {
        hidden::Shell::get(self, "KeyboardMode")
    }
    fn set_keyboard_mode(&self, value: i32) -> dbus::Result<()> {
        hidden::Shell::set(self, "KeyboardMode", value)
    }

    fn anchor(&self) -> dbus::Result<u32> {
        hidden::Shell::get(self, "Anchor")
    }
    fn set_anchor(&self, value: u32) -> dbus::Result<()> {
        hidden::Shell::set(self, "Anchor", value)
    }

    fn zindex(&self) -> dbus::Result<i32> {
        hidden::Shell::get(self, "ZIndex")
    }
    fn set_zindex(&self, value: i32) -> dbus::Result<()> {
        hidden::Shell::set(self, "ZIndex", value)
    }

    fn auto_exclusive_zone(&self) -> dbus::Result<bool> {
        hidden::Shell::get(self, "AutoExclusiveZone")
    }
    fn set_auto_exclusive_zone(&self, value: bool) -> dbus::Result<()> {
        hidden::Shell::set(self, "AutoExclusiveZone", value)
    }

    fn margin_top(&self) -> dbus::Result<i32> {
        hidden::Shell::get(self, "MarginTop")
    }
    fn set_margin_top(&self, value: i32) -> dbus::Result<()> {
        hidden::Shell::set(self, "MarginTop", value)
    }

    fn margin_right(&self) -> dbus::Result<i32> {
        hidden::Shell::get(self, "MarginRight")
    }
    fn set_margin_right(&self, value: i32) -> dbus::Result<()> {
        hidden::Shell::set(self, "MarginRight", value)
    }

    fn margin_bottom(&self) -> dbus::Result<i32> {
        hidden::Shell::get(self, "MarginBottom")
    }
    fn set_margin_bottom(&self, value: i32) -> dbus::Result<()> {
        hidden::Shell::set(self, "MarginBottom", value)
    }

    fn margin_left(&self) -> dbus::Result<i32> {
        hidden::Shell::get(self, "MarginLeft")
    }
    fn set_margin_left(&self, value: i32) -> dbus::Result<()> {
        hidden::Shell::set(self, "MarginLeft", value)
    }
}

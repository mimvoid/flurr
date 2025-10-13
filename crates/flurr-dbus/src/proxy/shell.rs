use crate::Window;

pub trait Shell: hidden::Shell {
    fn namespace(&self) -> dbus::Result<String>;

    fn layer(&self) -> dbus::Result<u8>;
    fn set_layer(&self, value: u8) -> dbus::Result<()>;

    fn keyboard_mode(&self) -> dbus::Result<u8>;
    fn set_keyboard_mode(&self, value: u8) -> dbus::Result<()>;

    fn anchor(&self) -> dbus::Result<u8>;
    fn set_anchor(&self, value: u8) -> dbus::Result<()>;

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
    fn namespace(&self) -> dbus::Result<String> {
        hidden::Shell::get(self, "Namespace")
    }

    fn layer(&self) -> dbus::Result<u8> {
        hidden::Shell::get(self, "Layer")
    }
    fn set_layer(&self, value: u8) -> dbus::Result<()> {
        hidden::Shell::set(self, "Layer", value)
    }

    fn keyboard_mode(&self) -> dbus::Result<u8> {
        hidden::Shell::get(self, "KeyboardMode")
    }
    fn set_keyboard_mode(&self, value: u8) -> dbus::Result<()> {
        hidden::Shell::set(self, "KeyboardMode", value)
    }

    fn anchor(&self) -> dbus::Result<u8> {
        hidden::Shell::get(self, "Anchor")
    }
    fn set_anchor(&self, value: u8) -> dbus::Result<()> {
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

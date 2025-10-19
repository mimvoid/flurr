use crate::Window;

pub trait Shell<'a, 'b>: hidden::Shell<'a, 'b> {
    fn namespace(&self) -> dbus::Result<String> {
        self.get("Namespace")
    }

    fn layer(&self) -> dbus::Result<u8> {
        self.get("Layer")
    }
    fn set_layer(&self, value: u8) -> dbus::Result<()> {
        self.set("Layer", value)
    }

    fn keyboard_mode(&self) -> dbus::Result<u8> {
        self.get("KeyboardMode")
    }
    fn set_keyboard_mode(&self, value: u8) -> dbus::Result<()> {
        self.set("KeyboardMode", value)
    }

    fn anchor(&self) -> dbus::Result<u8> {
        self.get("Anchor")
    }
    fn set_anchor(&self, value: u8) -> dbus::Result<()> {
        self.set("Anchor", value)
    }

    fn exclusion(&self) -> dbus::Result<i32> {
        self.get("Exclusion")
    }
    fn set_exclusion(&self, value: i32) -> dbus::Result<()> {
        self.set("Exclusion", value)
    }

    fn auto_exclusion(&self) -> dbus::Result<bool> {
        self.get("AutoExclusion")
    }
    fn set_auto_exclusion(&self, value: bool) -> dbus::Result<()> {
        self.set("AutoExclusion", value)
    }

    fn margin_top(&self) -> dbus::Result<i32> {
        self.get("MarginTop")
    }
    fn set_margin_top(&self, value: i32) -> dbus::Result<()> {
        self.set("MarginTop", value)
    }

    fn margin_right(&self) -> dbus::Result<i32> {
        self.get("MarginRight")
    }
    fn set_margin_right(&self, value: i32) -> dbus::Result<()> {
        self.set("MarginRight", value)
    }

    fn margin_bottom(&self) -> dbus::Result<i32> {
        self.get("MarginBottom")
    }
    fn set_margin_bottom(&self, value: i32) -> dbus::Result<()> {
        self.set("MarginBottom", value)
    }

    fn margin_left(&self) -> dbus::Result<i32> {
        self.get("MarginLeft")
    }
    fn set_margin_left(&self, value: i32) -> dbus::Result<()> {
        self.set("MarginLeft", value)
    }
}

mod hidden {
    pub trait Shell<'a, 'b>: crate::proxy::ProxyWrapper<'a, 'b> {
        crate::proxy::dbus_default_interface!("io.flurr.Shell");
    }
}

impl<'a, 'b> hidden::Shell<'a, 'b> for Window<'a, 'b> {}
impl<'a, 'b> Shell<'a, 'b> for Window<'a, 'b> {}

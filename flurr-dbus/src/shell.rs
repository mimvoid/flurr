use dbus::blocking::{Connection, Proxy, stdintf::org_freedesktop_dbus::Properties};

pub trait Shell {
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

impl<'a, 'b> Shell for Proxy<'a, &'b Connection> {
    fn namespace(&self) -> dbus::Result<String> {
        self.get("io.flurr.Shell", "Namespace")
    }

    fn layer(&self) -> dbus::Result<i32> {
        self.get("io.flurr.Shell", "Layer")
    }
    fn set_layer(&self, value: i32) -> dbus::Result<()> {
        self.set("io.flurr.Shell", "Layer", value)
    }

    fn keyboard_mode(&self) -> dbus::Result<i32> {
        self.get("io.flurr.Shell", "KeyboardMode")
    }
    fn set_keyboard_mode(&self, value: i32) -> dbus::Result<()> {
        self.set("io.flurr.Shell", "KeyboardMode", value)
    }

    fn anchor(&self) -> dbus::Result<u32> {
        self.get("io.flurr.Shell", "Anchor")
    }
    fn set_anchor(&self, value: u32) -> dbus::Result<()> {
        self.set("io.flurr.Shell", "Anchor", value)
    }

    fn zindex(&self) -> dbus::Result<i32> {
        self.get("io.flurr.Shell", "ZIndex")
    }
    fn set_zindex(&self, value: i32) -> dbus::Result<()> {
        self.set("io.flurr.Shell", "ZIndex", value)
    }

    fn auto_exclusive_zone(&self) -> dbus::Result<bool> {
        self.get("io.flurr.Shell", "AutoExclusiveZone")
    }
    fn set_auto_exclusive_zone(&self, value: bool) -> dbus::Result<()> {
        self.set("io.flurr.Shell", "AutoExclusiveZone", value)
    }

    fn margin_top(&self) -> dbus::Result<i32> {
        self.get("io.flurr.Shell", "MarginTop")
    }
    fn set_margin_top(&self, value: i32) -> dbus::Result<()> {
        self.set("io.flurr.Shell", "MarginTop", value)
    }

    fn margin_right(&self) -> dbus::Result<i32> {
        self.get("io.flurr.Shell", "MarginRight")
    }
    fn set_margin_right(&self, value: i32) -> dbus::Result<()> {
        self.set("io.flurr.Shell", "MarginRight", value)
    }

    fn margin_bottom(&self) -> dbus::Result<i32> {
        self.get("io.flurr.Shell", "MarginBottom")
    }
    fn set_margin_bottom(&self, value: i32) -> dbus::Result<()> {
        self.set("io.flurr.Shell", "MarginBottom", value)
    }

    fn margin_left(&self) -> dbus::Result<i32> {
        self.get("io.flurr.Shell", "MarginLeft")
    }
    fn set_margin_left(&self, value: i32) -> dbus::Result<()> {
        self.set("io.flurr.Shell", "MarginLeft", value)
    }
}

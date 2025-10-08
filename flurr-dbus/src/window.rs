use dbus::blocking::{Connection, Proxy, stdintf::org_freedesktop_dbus::Properties};

pub trait Window {
    fn name(&self) -> dbus::Result<String>;

    fn visible(&self) -> dbus::Result<bool>;
    fn set_visible(&self, value: bool) -> dbus::Result<()>;

    fn toggle(&self) -> dbus::Result<()>;
}

impl<'a, 'b> Window for Proxy<'a, &'b Connection> {
    fn toggle(&self) -> dbus::Result<()> {
        self.method_call("io.flurr.Window", "Toggle", ())
    }

    fn name(&self) -> dbus::Result<String> {
        self.get("io.flurr.Window", "Name")
    }

    fn visible(&self) -> dbus::Result<bool> {
        self.get("io.flurr.Wiindow", "Visible")
    }
    fn set_visible(&self, value: bool) -> dbus::Result<()> {
        self.set("io.flurr.Window", "Visible", value)
    }
}

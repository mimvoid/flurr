use dbus::blocking::{Connection, Proxy, stdintf::org_freedesktop_dbus::Properties};

pub trait PinShell {
    fn unlocked(&self) -> dbus::Result<bool>;
    fn set_unlocked(&self, value: bool) -> dbus::Result<()>;
}

impl<'a, 'b> PinShell for Proxy<'a, &'b Connection> {
    fn unlocked(&self) -> Result<bool, dbus::Error> {
        self.get("io.flurr.PinShell", "Unlocked")
    }

    fn set_unlocked(&self, value: bool) -> Result<(), dbus::Error> {
        self.set("io.flurr.PinShell", "Unlocked", value)
    }
}

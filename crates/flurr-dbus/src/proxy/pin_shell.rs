use crate::Window;

pub trait PinShell<'a, 'b>: hidden::PinShell<'a, 'b> {
    fn unlocked(&self) -> dbus::Result<bool> {
        self.get("Unlocked")
    }
    fn set_unlocked(&self, value: bool) -> dbus::Result<()> {
        self.set("Unlocked", value)
    }
}

mod hidden {
    pub trait PinShell<'a, 'b>: crate::proxy::ProxyWrapper<'a, 'b> {
        crate::proxy::dbus_default_interface!("io.flurr.PinShell");
    }
}

impl<'a, 'b> hidden::PinShell<'a, 'b> for Window<'a, 'b> {}
impl<'a, 'b> PinShell<'a, 'b> for Window<'a, 'b> {}

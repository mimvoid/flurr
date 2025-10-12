use crate::Window;

pub trait PinShell: hidden::PinShell {
    fn unlocked(&self) -> dbus::Result<bool>;
    fn set_unlocked(&self, value: bool) -> dbus::Result<()>;
}

mod hidden {
    pub trait PinShell {
        crate::proxy::dbus_default_trait!();
    }
}
impl<'a, 'b> hidden::PinShell for Window<'a, 'b> {
    super::dbus_default_interface!("io.flurr.PinShell");
}

impl<'a, 'b> PinShell for Window<'a, 'b> {
    fn unlocked(&self) -> dbus::Result<bool> {
        hidden::PinShell::get(self, "Unlocked")
    }
    fn set_unlocked(&self, value: bool) -> dbus::Result<()> {
        hidden::PinShell::set(self, "Unlocked", value)
    }
}

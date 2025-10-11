use crate::Window;

pub trait PinShell: hidden::PinShell {
    fn get_all(&self) -> dbus::Result<dbus::arg::PropMap>;

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
    super::dbus_get_all!("io.flurr.PinShell");

    fn unlocked(&self) -> Result<bool, dbus::Error> {
        hidden::PinShell::get(self, "Unlocked")
    }
    fn set_unlocked(&self, value: bool) -> Result<(), dbus::Error> {
        hidden::PinShell::set(self, "Unlocked", value)
    }
}

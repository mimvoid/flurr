use crate::Window;

pub trait PinShell {
    fn get_all(&self) -> dbus::Result<dbus::arg::PropMap>;

    fn unlocked(&self) -> dbus::Result<bool>;
    fn set_unlocked(&self, value: bool) -> dbus::Result<()>;
}

mod private {
    pub trait PinShell {
        crate::proxy::dbus_default_trait!();
    }
    impl<'a, 'b> PinShell for super::Window<'a, 'b> {
        crate::proxy::dbus_default_interface!("io.flurr.PinShell");
    }
}

impl<'a, 'b> PinShell for Window<'a, 'b> {
    super::dbus_get_all!("io.flurr.PinShell");

    fn unlocked(&self) -> Result<bool, dbus::Error> {
        private::PinShell::get(self, "Unlocked")
    }
    fn set_unlocked(&self, value: bool) -> Result<(), dbus::Error> {
        private::PinShell::set(self, "Unlocked", value)
    }
}

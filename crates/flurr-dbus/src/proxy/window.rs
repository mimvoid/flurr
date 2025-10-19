use dbus::blocking::{Connection, Proxy};

use super::{TIMEOUT, dbus_default_interface, make_dest, ping};
use crate::make_window_path;

pub struct Window<'a, 'b> {
    pub proxy: Proxy<'a, &'b Connection>,
}

impl<'a, 'b> Window<'a, 'b> {
    dbus_default_interface!("io.flurr.Window");
    ping!(pub);

    pub fn new(connection: &'b Connection, instance_name: &str, id: &u32) -> Self {
        Self {
            proxy: connection.with_proxy(
                make_dest(instance_name),
                make_window_path(instance_name, id),
                TIMEOUT,
            ),
        }
    }

    pub fn with_path<P>(connection: &'b Connection, instance_name: &str, path: P) -> Self
    where
        P: Into<dbus::Path<'a>>,
    {
        Self {
            proxy: connection.with_proxy(make_dest(instance_name), path, TIMEOUT),
        }
    }

    pub fn toggle(&self) -> dbus::Result<()> {
        self.method_call("Toggle", ())
    }

    pub fn name(&self) -> dbus::Result<String> {
        self.get("Name")
    }

    pub fn visible(&self) -> dbus::Result<bool> {
        self.get("Visible")
    }
    pub fn set_visible(&self, value: bool) -> dbus::Result<()> {
        self.set("Visible", value)
    }
}

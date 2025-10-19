use super::{ProxyWrapper, TIMEOUT, dbus_default_interface, make_dest, make_object_path};
use dbus::blocking::{Connection, Proxy};

pub struct Application<'a, 'b>(Proxy<'a, &'b Connection>);

impl<'a, 'b> ProxyWrapper<'a, 'b> for Application<'a, 'b> {
    fn proxy(&self) -> &Proxy<'a, &'b Connection> {
        &self.0
    }
}

impl<'a, 'b> Application<'a, 'b> {
    dbus_default_interface!("io.flurr.Application");

    pub fn new(connection: &'b Connection, instance_name: &str) -> Self {
        Self(connection.with_proxy(
            make_dest(instance_name),
            make_object_path(instance_name),
            TIMEOUT,
        ))
    }

    pub fn get_window_path(&self, window_name: &str) -> dbus::Result<dbus::Path<'static>> {
        self.method_map("GetWindowPath", (window_name,))
    }

    pub fn list_window_names(&self) -> dbus::Result<Vec<String>> {
        self.method_map("ListWindowNames", ())
    }

    pub fn list_window_ids(&self) -> dbus::Result<Vec<u32>> {
        self.method_map("ListWindowIds", ())
    }

    pub fn list_window_paths(&self) -> dbus::Result<Vec<dbus::Path<'static>>> {
        self.method_map("ListWindowPaths", ())
    }

    pub fn quit(&self) -> dbus::Result<()> {
        self.method_call("Quit", ())
    }
}

use dbus::blocking::{Connection, Proxy};

pub trait Application {
    fn get_window_path(&self, window_name: &str) -> dbus::Result<dbus::Path<'static>>;
    fn list_window_names(&self) -> dbus::Result<Vec<String>>;
    fn list_window_ids(&self) -> dbus::Result<Vec<u32>>;
    fn list_window_paths(&self) -> dbus::Result<Vec<dbus::Path<'static>>>;
    fn quit(&self) -> dbus::Result<()>;
}

impl<'a, 'b> Application for Proxy<'a, &'b Connection> {
    fn get_window_path(&self, window_name: &str) -> dbus::Result<dbus::Path<'static>> {
        self.method_call("io.flurr.Application", "GetWindowPath", (window_name,))
            .map(|r: (dbus::Path<'static>,)| r.0)
    }

    fn list_window_names(&self) -> dbus::Result<Vec<String>> {
        self.method_call("io.flurr.Application", "ListWindowNames", ())
            .map(|r: (Vec<String>,)| r.0)
    }

    fn list_window_ids(&self) -> dbus::Result<Vec<u32>> {
        self.method_call("io.flurr.Application", "ListWindowIds", ())
            .map(|r: (Vec<u32>,)| r.0)
    }

    fn list_window_paths(&self) -> dbus::Result<Vec<dbus::Path<'static>>> {
        self.method_call("io.flurr.Application", "ListWindowPaths", ())
            .map(|r: (Vec<dbus::Path<'static>>,)| r.0)
    }

    fn quit(&self) -> dbus::Result<()> {
        self.method_call("io.flurr.Application", "Quit", ())
    }
}

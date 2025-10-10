mod application;
mod pin_shell;
mod shell;
mod window;

pub use application::Application;
pub use pin_shell::PinShell;
pub use shell::Shell;
pub use window::Window;

const TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

fn make_dest(instance_name: &str) -> String {
    ["io.flurr.", instance_name].concat()
}

fn make_object_path(instance_name: &str) -> String {
    ["/io/flurr/", instance_name].concat()
}

macro_rules! dbus_default_interface {
    ($name: expr) => {
        #[allow(dead_code)]
        fn get<R>(&self, name: &str) -> dbus::Result<R>
        where
            R: for<'r> dbus::arg::Get<'r> + 'static,
        {
            use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
            self.proxy.get($name, name)
        }

        #[allow(dead_code)]
        fn set<A>(&self, name: &str, value: A) -> dbus::Result<()>
        where
            A: dbus::arg::Arg + dbus::arg::Append,
        {
            use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
            self.proxy.set($name, name, value)
        }

        #[allow(dead_code)]
        fn method_call<A, R>(&self, name: &str, args: A) -> dbus::Result<R>
        where
            A: dbus::arg::AppendAll,
            R: dbus::arg::ReadAll,
        {
            self.proxy.method_call($name, name, args)
        }

        #[allow(dead_code)]
        fn method_map<A, R>(&self, name: &str, args: A) -> dbus::Result<R>
        where
            A: dbus::arg::AppendAll,
            R: for<'r> dbus::arg::Get<'r> + dbus::arg::Arg,
        {
            self.proxy.method_call($name, name, args).map(|r: (R,)| r.0)
        }
    };
}
pub(super) use dbus_default_interface;

macro_rules! dbus_default_trait {
    () => {
        #[allow(dead_code)]
        fn get<R: for<'r> dbus::arg::Get<'r> + 'static>(&self, name: &str) -> dbus::Result<R>;

        #[allow(dead_code)]
        fn set<A>(&self, name: &str, value: A) -> dbus::Result<()>
        where
            A: dbus::arg::Arg + dbus::arg::Append;

        #[allow(dead_code)]
        fn method_call<A, R>(&self, name: &str, args: A) -> dbus::Result<R>
        where
            A: dbus::arg::AppendAll,
            R: dbus::arg::ReadAll;

        #[allow(dead_code)]
        fn method_map<A, R>(&self, name: &str, args: A) -> dbus::Result<R>
        where
            A: dbus::arg::AppendAll,
            R: for<'r> dbus::arg::Get<'r> + dbus::arg::Arg;
    };
}
pub(super) use dbus_default_trait;

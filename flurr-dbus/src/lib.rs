mod application;
mod pin_shell;
mod shell;
mod window;

pub use application::Application;
pub use pin_shell::PinShell;
pub use shell::Shell;
pub use window::Window;

use dbus::blocking::{Connection, Proxy};

pub fn application_proxy<'a, 'b>(
    connection: &'b Connection,
    instance_name: &str,
) -> Proxy<'a, &'b Connection> {
    connection.with_proxy(
        make_dest(instance_name),
        make_object_path(instance_name),
        TIMEOUT,
    )
}

pub fn window_proxy<'a, 'b>(
    connection: &'b Connection,
    instance_name: &str,
    id: &u32,
) -> Proxy<'a, &'b Connection> {
    connection.with_proxy(
        make_dest(instance_name),
        make_object_path(instance_name) + "/window/" + id.to_string().as_str(),
        TIMEOUT,
    )
}

const TIMEOUT: std::time::Duration = std::time::Duration::from_millis(5000);

fn make_dest(instance_name: &str) -> String {
    "io.flurr.".to_owned() + instance_name
}

fn make_object_path(instance_name: &str) -> String {
    "/io/flurr/".to_owned() + instance_name
}

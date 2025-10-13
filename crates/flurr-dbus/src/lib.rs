mod proxy;
pub use proxy::{Application, PinShell, Shell, Window};

pub mod props;

pub fn make_window_path(instance_name: &str, id: &u32) -> dbus::Path<'static> {
    dbus::Path::from(format!("/io/flurr/{instance_name}/window/{id}"))
}

mod proxy;
mod enums;

pub use proxy::Application;
pub use proxy::PinShell;
pub use proxy::Shell;
pub use proxy::Window;

pub use enums::{Anchor, Layer, KeyboardMode};

pub fn make_window_path(instance_name: &str, id: &u32) -> dbus::Path<'static> {
    dbus::Path::from(format!("/io/flurr/{instance_name}/window/{id}"))
}

use super::{BlockingProperties, PinShellProps, PropertyError, ShellProps, get_cast, parse_string};
use dbus::arg::PropMap;

#[derive(Debug, Default)]
pub struct WindowProps {
    pub name: String,
    pub visible: bool,
    shell: Option<ShellProps>,
    pin_shell: Option<PinShellProps>,
}

impl WindowProps {
    /// With a blocking DBus proxy, fetch and parse the properties for the io.flurr.Window interface,
    /// as well as for io.flurr.Shell and io.flurr.PinShell if available.
    pub fn get_blocking(proxy: &impl BlockingProperties) -> Result<Self, PropertyError> {
        let window = proxy.get_all("io.flurr.Window")?;
        let shell = proxy.get_all("io.flurr.Shell").ok();
        let pin_shell = proxy.get_all("io.flurr.PinShell").ok();

        Self::from_prop_maps(&window, shell.as_ref(), pin_shell.as_ref())
    }

    /// Parse a DBus property map, assumed to only contain properties for io.flurr.Window.
    pub fn from_prop_map(props: &PropMap) -> Result<Self, PropertyError> {
        Self::from_prop_maps(props, None, None)
    }

    /// Parse DBus property maps for the interfaces io.flurr.Window, io.flurr.Shell, and
    /// io.flurr.PinShell.
    pub fn from_prop_maps(
        window: &PropMap,
        shell: Option<&PropMap>,
        pin_shell: Option<&PropMap>,
    ) -> Result<Self, PropertyError> {
        let shell_props = match shell {
            Some(shell_map) => Some(ShellProps::from_prop_map(shell_map)?),
            None => None,
        };
        let pin_shell_props = match pin_shell {
            Some(pin_shell_map) => Some(PinShellProps::from_prop_map(pin_shell_map)?),
            None => None,
        };

        Ok(WindowProps {
            name: parse_string!("io.flurr.Window", window, "Name"),
            visible: get_cast!("io.flurr.Window", window, "Visible", bool),
            shell: shell_props,
            pin_shell: pin_shell_props,
        })
    }

    /// Properties from io.flurr.Shell, if any.
    pub fn shell(&self) -> Option<&ShellProps> {
        self.shell.as_ref()
    }

    /// Properties from io.flurr.PinShell, if any.
    pub fn pin_shell(&self) -> Option<&PinShellProps> {
        self.pin_shell.as_ref()
    }
}

impl TryFrom<dbus::arg::PropMap> for WindowProps {
    type Error = PropertyError;
    fn try_from(value: dbus::arg::PropMap) -> Result<Self, Self::Error> {
        Self::from_prop_map(&value)
    }
}

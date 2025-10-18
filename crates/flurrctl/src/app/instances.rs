use dbus::blocking::Connection;
use std::io::{Write, stdout};

/// Finds all DBus service names and prints any that begin with "io.flurr."
pub fn list_instances(conn: &Connection) -> crate::Result<()> {
    let proxy = conn.with_proxy(
        "org.freedesktop.DBus",
        "/org/freedesktop/DBus",
        std::time::Duration::from_secs(5),
    );

    log::info!("Querying org.freedesktop.DBus for instances");
    let (names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus", "ListNames", ())?;

    let instances = names
        .iter()
        .filter_map(|name| name.strip_prefix("io.flurr."));

    let mut count = 0u8; // if you even have 20+ Flurr instances running, I'd be concerned
    let mut lock = stdout().lock();

    for instance in instances {
        writeln!(lock, "{instance}")?;
        count += 1;
    }

    log::info!("Found {count} Flurr instances");

    Ok(())
}

use clap::Parser;
use dbus::blocking::Connection;
use std::io::{Write, stdout};
use std::process::ExitCode;

mod error;
use error::{DBusError, Error, Result};

mod args;
use args::Commands;

mod window;
use window::{hide_window, show_window, toggle_window};

fn main() -> ExitCode {
    let args = args::Cli::parse();

    env_logger::Builder::new()
        .filter_level(args.verbosity.into())
        .format_timestamp(None)
        .format_target(cfg!(debug_assertions))
        .init();

    log::info!("Connecting to DBus");
    let Ok(conn) = Connection::new_session() else {
        log::error!("Failed to connect to DBus");
        return ExitCode::FAILURE;
    };

    let res = match &args.subcommand {
        Commands::Toggle(win) => toggle_window(&conn, args.instance.as_str(), &win),
        Commands::Show(win) => show_window(&conn, args.instance.as_str(), &win),
        Commands::Hide(win) => hide_window(&conn, args.instance.as_str(), &win),
        Commands::Instances => list_instances(&conn),
    };

    if let Err(err) = res {
        log::error!("{}", err);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn list_instances(conn: &Connection) -> Result<()> {
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

    let mut count: u8 = 0; // if you even have 20+ Flurr instances running, I'd be concerned
    let mut lock = stdout().lock();

    for instance in instances {
        let _ = writeln!(lock, "{}", instance);
        count += 1;
    }

    log::info!("Found {} Flurr instances", count);

    Ok(())
}

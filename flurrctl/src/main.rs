use clap::Parser;
use dbus::blocking::Connection;
use std::process::ExitCode;

mod error;

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

    if let Err(dbus_err) = res {
        match error::FlurrctlError::new(args.instance, &dbus_err) {
            Some(err) => log::error!("{err}"),
            None => log::error!("{dbus_err}")
        };

        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn list_instances(conn: &Connection) -> dbus::Result<()> {
    let proxy = conn.with_proxy(
        "org.freedesktop.DBus",
        "/org/freedesktop/DBus",
        std::time::Duration::from_millis(5000),
    );

    log::info!("Querying org.freedesktop.DBus for instance names");
    let (names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus", "ListNames", ())?;

    let instances: Vec<String> = names
        .iter()
        .filter_map(|name| name.strip_prefix("io.flurr.").map(str::to_owned))
        .collect();

    log::info!("Found {} Flurr instances", instances.len());
    if !instances.is_empty() {
        println!("{}", instances.join("\n"));
    }

    Ok(())
}

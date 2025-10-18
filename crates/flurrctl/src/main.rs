use clap::Parser;
use dbus::blocking::Connection;
use std::process::ExitCode;

mod error;
use error::{DBusError, Error, Result};

mod args;
use args::Commands;

mod app;
mod window;

use app::{list_instances, quit};
use window::{hide_window, print_windows, show_window, toggle_window};

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
        Commands::Toggle(win) => toggle_window(&conn, args.instance.as_str(), win.window.as_str()),
        Commands::Show(win) => show_window(&conn, args.instance.as_str(), win.window.as_str()),
        Commands::Hide(win) => hide_window(&conn, args.instance.as_str(), win.window.as_str()),
        Commands::Windows => print_windows(&conn, args.instance.as_str()),
        Commands::Quit => quit(&conn, args.instance.as_str()),
        Commands::Instances => list_instances(&conn),
    };

    if let Err(err) = res {
        log::error!("{err}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

use clap::Parser;
use dbus::blocking::Connection;

mod args;
use args::Commands;

mod window;
use window::{toggle_window, show_window, hide_window};

const TIMEOUT: std::time::Duration = std::time::Duration::from_millis(5000);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Cli::parse();

    match &args.subcommand {
        Commands::Toggle(win) => toggle_window(args.instance.as_str(), &win)?,
        Commands::Show(win) => show_window(args.instance.as_str(), &win)?,
        Commands::Hide(win) => hide_window(args.instance.as_str(), &win)?,
        Commands::Instances => list_instances()?,
    };

    Ok(())
}

fn list_instances() -> dbus::Result<()> {
    let conn = Connection::new_session()?;
    let proxy = conn.with_proxy(
        "org.freedesktop.DBus",
        "/org/freedesktop/DBus",
        TIMEOUT,
    );
    let (names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus", "ListNames", ())?;

    let instances: Vec<String> = names
        .iter()
        .filter_map(|name| name.strip_prefix("io.flurr.").map(str::to_owned))
        .collect();

    if instances.is_empty() {
        println!("No Flurr instances found");
    } else {
        println!("{}", instances.join("\n"));
    }

    Ok(())
}

use clap::Parser;
use dbus::blocking::Connection;
use std::time::Duration;

mod args;
use args::{Commands, WindowCommand};

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
        Duration::from_millis(5000),
    );
    let (names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus", "ListNames", ())?;

    let instances: Vec<String> = names
        .iter()
        .filter_map(|name| {
            if let Some(instance) = name.strip_prefix("io.flurr.") {
                Some(String::from(instance))
            } else {
                None
            }
        })
        .collect();

    if instances.is_empty() {
        println!("No Flurr instances found");
    } else {
        println!("{}", instances.join("\n"));
    }

    Ok(())
}

macro_rules! with_window_methods {
    ($name: ident, $app_method: expr, $win_method: expr) => {
        fn $name(instance: &str, opts: &WindowCommand) -> dbus::Result<()> {
            window_cmd(instance, opts, $app_method, $win_method)
        }
    };
}
with_window_methods!(toggle_window, "ToggleWindow", "Toggle");
with_window_methods!(show_window, "ShowWindow", "Show");
with_window_methods!(hide_window, "HideWindow", "Hide");

fn window_cmd(
    instance: &str,
    opts: &WindowCommand,
    app_method: &str,
    win_method: &str,
) -> dbus::Result<()> {
    let conn = Connection::new_session()?;

    if let Some(name) = opts.name.as_deref() {
        let proxy = conn.with_proxy(
            format!("io.flurr.{instance}"),
            format!("/io/flurr/{instance}"),
            Duration::from_millis(5000),
        );

        let _: () = proxy.method_call("io.flurr.Application", app_method, (name,))?;
    } else if let Some(id) = opts.id {
        let proxy = conn.with_proxy(
            format!("io.flurr.{instance}"),
            format!("/io/flurr/{instance}/window/{id}"),
            Duration::from_millis(5000),
        );

        let _: () = proxy.method_call("io.flurr.Window", win_method, ())?;
    }

    Ok(())
}

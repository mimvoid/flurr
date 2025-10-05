use clap::Parser;
use dbus::blocking::Connection;
use std::time::Duration;

mod args;
use args::{Commands, WindowCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Cli::parse();
    let instance = args.instance;

    match &args.subcommand {
        Commands::Toggle(win) => toggle_window(instance.as_str(), &win),
    }?;

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

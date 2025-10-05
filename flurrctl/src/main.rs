use argh::FromArgs;
use dbus::blocking::Connection;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = argh::from_env();
    if args.version {
        println!(env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let instance = args.instance;
    let dest = format!("io.flurr.{instance}");

    let conn = Connection::new_session()?;
    let app_proxy = conn.with_proxy(
        dest,
        format!("/io/flurr/{instance}"),
        Duration::from_millis(5000),
    );

    match args.nested {
        Subcommands::Toggle(opts) => {
            let _: () = app_proxy.method_call(
                "io.flurr.Application",
                "ToggleWindow",
                (opts.window_name,),
            )?;
        }
    }

    Ok(())
}

#[derive(FromArgs, PartialEq, Debug)]
/// Widget system
struct Args {
    /// print the version and exit
    #[argh(switch)]
    version: bool,

    /// the instance name, defaults to "Flurr"
    #[argh(option, short = 'i', default = "String::from(\"Flurr\")")]
    instance: String,

    #[argh(subcommand)]
    nested: Subcommands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommands {
    Toggle(SubToggle),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Toggle a window by name
#[argh(subcommand, name = "toggle")]
struct SubToggle {
    #[argh(positional)]
    /// the name of the window
    window_name: String,
}

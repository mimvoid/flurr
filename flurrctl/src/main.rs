use gtk::{glib, gio::ApplicationCommandLine};
use gtk::{prelude::*, Application};

fn main() -> glib::ExitCode {
    let args: Args = argh::from_env();
    if args.version {
        println!(env!("CARGO_PKG_VERSION"));
        return glib::ExitCode::SUCCESS;
    }

    let app = Application::builder()
        .application_id("org.flurr.Flurr")
        .build();

    app.connect_activate(activate);
    app.connect_command_line(cli_command);

    app.run()
}

#[derive(argh::FromArgs, Debug)]
/// Widget system
struct Args {
    /// print the version and exit
    #[argh(switch)]
    version: bool,
}

fn activate(_app: &Application) {

}

fn cli_command(_app: &Application, _app_cli: &ApplicationCommandLine) -> glib::ExitCode {
    let args: Args = argh::from_env();
    if args.version {
        println!(env!("CARGO_PKG_VERSION"));
        return glib::ExitCode::SUCCESS
    }

    glib::ExitCode::SUCCESS
}

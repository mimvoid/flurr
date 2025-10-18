use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Name of the Flurr instance
    #[arg(short, long, global = true, default_value = "Flurr")]
    pub instance: String,

    #[command(flatten)]
    pub verbosity: Verbosity,

    #[command(subcommand)]
    pub subcommand: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Toggle the visibility of a window
    Toggle(WindowCommand),
    /// Make a window visible
    Show(WindowCommand),
    /// Make a window invisible
    Hide(WindowCommand),
    /// Print windows and their properties
    Get(GetCommand),
    /// Print registered Flurr instances
    Instances,
    /// Quit an instance
    Quit,
}

#[derive(Args)]
pub struct WindowCommand {
    /// The window name, or the id if a number
    pub window: String,
}

#[derive(Args)]
pub struct Verbosity {
    /// Silence all logs
    #[arg(short, long, default_value_t = false, global = true)]
    pub quiet: bool,

    /// Enable verbose logging
    #[arg(short, long, default_value_t = false, global = true)]
    pub verbose: bool,
}

impl Into<log::LevelFilter> for Verbosity {
    fn into(self) -> log::LevelFilter {
        if self.quiet {
            log::LevelFilter::Off
        } else if self.verbose {
            log::LevelFilter::Info
        } else {
            log::LevelFilter::Error
        }
    }
}

#[derive(Args)]
pub struct GetCommand {
    /// The names or ids of the windows to print
    pub windows: Vec<String>,

    /// Print enums and bitflags as numbers instead of strings
    #[arg(short, long)]
    pub raw: bool,
}

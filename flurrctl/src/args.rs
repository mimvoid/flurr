use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Name of the Flurr instance
    #[arg(short, long, default_value_t = String::from("Flurr"))]
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
    /// Print registered Flurr instances
    Instances,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct WindowCommand {
    /// The window name
    #[arg(short, long)]
    pub name: Option<String>,

    /// The application window id
    #[arg(short = 'I', long)]
    pub id: Option<u32>,
}

#[derive(Args)]
pub struct Verbosity {
    /// Silence all logs
    #[arg(short, long, default_value_t = false)]
    pub quiet: bool,

    /// Enable verbose logging
    #[arg(long, default_value_t = false)]
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

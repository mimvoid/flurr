use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub verbosity: Verbosity,

    #[command(subcommand)]
    pub subcommand: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Toggle the visibility of a window
    Toggle(WindowCommand),
    /// Print windows and their properties
    Get(GetCommand),
    /// Set properties for a window
    Set(SetCommand),
    /// Print registered Flurr instances
    Instances,
    /// Quit an instance
    Quit {
        /// Name(s) of the Flurr instances
        instances: Vec<String>,
    },
}

#[derive(Args)]
pub struct WindowCommand {
    /// The window name, or the id if a number
    pub window: String,

    /// Name of the Flurr instance
    #[arg(short, long, default_value = "Flurr")]
    pub instance: String,
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
            log::LevelFilter::Warn
        }
    }
}

#[derive(Args)]
pub struct GetCommand {
    /// The names or ids of the windows to print
    pub windows: Vec<String>,

    /// Name of the Flurr instance
    #[arg(short, long, default_value = "Flurr")]
    pub instance: String,

    /// Print enums and bitflags as numbers instead of strings
    #[arg(short, long)]
    pub raw: bool,
}

#[derive(Args)]
pub struct SetCommand {
    /// Name or id of the window to write to
    pub window: String,

    /// Name of the Flurr instance
    #[arg(short, long, default_value = "Flurr")]
    pub instance: String,

    /// Whether the window can be seen
    #[arg(long, hide_possible_values = true)]
    pub visible: Option<bool>,

    /// Shell component layer [ex: BACKGROUND, 3]
    #[arg(long)]
    pub layer: Option<String>,
    /// Keyboard input sensitivity [ex: ON_DEMAND, 2]
    #[arg(long)]
    pub keyboard_mode: Option<String>,
    /// Screen edges to anchor to [ex: "TOP | LEFT", 13]
    #[arg(long)]
    pub anchor: Option<String>,
    /// Shell exclusion zone, conflicts with --auto-exclusion
    #[arg(long)]
    pub exclusion: Option<i32>,
    /// Automatically exclude the window with margins
    #[arg(long, hide_possible_values = true, conflicts_with = "exclusion")]
    pub auto_exclusion: Option<bool>,
    /// Margin for top anchor
    #[arg(long)]
    pub margin_top: Option<i32>,
    /// Margin for right anchor
    #[arg(long)]
    pub margin_right: Option<i32>,
    /// Margin for bottom anchor
    #[arg(long)]
    pub margin_bottom: Option<i32>,
    /// Margin for left anchor
    #[arg(long)]
    pub margin_left: Option<i32>,

    /// Whether the window can be dragged and moved
    #[arg(long, hide_possible_values = true)]
    pub unlocked: Option<bool>,
}

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Name of the Flurr instance
    #[arg(short, long, default_value_t = String::from("Flurr"))]
    pub instance: String,

    #[command(subcommand)]
    pub subcommand: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Toggle the visibility of a window
    Toggle(WindowCommand),
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

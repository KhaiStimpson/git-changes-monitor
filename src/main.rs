mod app;
mod config;
mod event;
mod git;
mod theme;
mod tui;
mod ui;
mod watcher;

use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre::Result;

use crate::app::App;
use crate::config::loader::ConfigLoader;
use crate::theme::themes::Theme;

/// Git File Monitor - A TUI for monitoring Git repository changes in real-time
#[derive(Parser, Debug)]
#[command(name = "gfm")]
#[command(version = "0.1.0")]
#[command(about = "Monitor Git repository changes in real-time with a beautiful TUI")]
struct Cli {
    /// Path to the Git repository to monitor
    #[arg(default_value = ".")]
    directory: PathBuf,

    /// Path to custom configuration file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Disable watch mode (single snapshot)
    #[arg(long = "no-watch")]
    no_watch: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize error handling
    color_eyre::install()?;

    // Parse CLI arguments
    let cli = Cli::parse();

    // Resolve repository path
    let repo_path = if cli.directory.is_absolute() {
        cli.directory
    } else {
        std::env::current_dir()?.join(&cli.directory)
    };

    // Load configuration
    let config_loader = ConfigLoader::new(cli.config)?;
    let config = config_loader.load()?;

    // Get theme based on config
    let theme = Theme::from_name(&config.ui.color_scheme);

    // Create and run the application
    let mut app = App::new(repo_path, config, theme, !cli.no_watch);
    app.run().await?;

    Ok(())
}

use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod preset;
mod symlink;

#[derive(Parser)]
#[command(name = "stampenv")]
#[command(about = "A CLI tool for deploying preset-based folder/file structures via symlinks")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create symlinks from a preset to the target path
    Link {
        /// Name of the preset to apply
        preset: String,
        /// Target path (default: current directory)
        target: Option<String>,
        /// Proceed without confirmation
        #[arg(short, long)]
        yes: bool,
        /// Add to sync tracking list (auto-sync target for commit --sync)
        #[arg(short, long)]
        sync: bool,
        /// Run quietly without output
        #[arg(short, long)]
        quiet: bool,
    },
    /// Remove created symlinks
    Unlink {
        /// Target path (default: current directory)
        target: Option<String>,
        /// Run quietly without output
        #[arg(short, long)]
        quiet: bool,
    },
    /// Save current structure as a preset
    Commit {
        /// Preset name
        name: String,
        /// Filter files with glob patterns (e.g., "*.rs", "src/**/*.toml")
        #[arg(short, long)]
        patterns: Option<Vec<String>>,
        /// Sync changes to already linked locations
        #[arg(short, long)]
        sync: bool,
        /// Run quietly without output
        #[arg(short = 'q', long)]
        quiet: bool,
    },
    /// List saved presets
    List,
    /// Display preset contents
    Show {
        /// Name of the preset to show
        preset: String,
    },
    /// Delete saved presets
    Delete {
        /// Names of presets to delete (supports multiple)
        #[arg(required = true)]
        presets: Vec<String>,
        /// Proceed without confirmation
        #[arg(short, long)]
        yes: bool,
        /// Also unlink from all locations where these presets are linked
        #[arg(long)]
        unlink: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Link {
            preset,
            target,
            yes,
            sync,
            quiet,
        } => {
            commands::link::run(&preset, target.as_deref(), yes, sync, quiet)?;
        }
        Commands::Unlink { target, quiet } => {
            commands::unlink::run(target.as_deref(), quiet)?;
        }
        Commands::Commit {
            name,
            patterns,
            sync,
            quiet,
        } => {
            commands::commit::run(&name, patterns.as_deref(), sync, quiet)?;
        }
        Commands::List => {
            commands::list::run()?;
        }
        Commands::Show { preset } => {
            commands::list::show(&preset)?;
        }
        Commands::Delete {
            presets,
            yes,
            unlink,
        } => {
            commands::delete::run(&presets, yes, unlink)?;
        }
    }

    Ok(())
}

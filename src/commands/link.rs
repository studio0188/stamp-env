use anyhow::{Context, Result};
use std::io::{self, Write};
use std::path::Path;

use crate::preset::manager::PresetManager;
use crate::symlink::builder::SymlinkBuilder;

/// Check if a directory is empty
fn is_directory_empty(path: &Path) -> Result<bool> {
    if !path.exists() {
        return Ok(true);
    }

    if !path.is_dir() {
        return Ok(false);
    }

    let mut entries = std::fs::read_dir(path)?;
    Ok(entries.next().is_none())
}

/// Prompt user for confirmation
fn confirm(message: &str) -> Result<bool> {
    print!("{} [y/N]: ", message);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_lowercase();
    Ok(input == "y" || input == "yes")
}

pub fn run(preset_name: &str, target: Option<&str>, skip_confirm: bool, sync: bool, quiet: bool) -> Result<()> {
    let target_path = target.unwrap_or(".");
    let target = Path::new(target_path);

    // Prompt for confirmation if directory is not empty
    if !skip_confirm && !is_directory_empty(target)? {
        let message = format!(
            "Target directory '{}' is not empty. Do you want to continue?",
            target_path
        );
        if !confirm(&message)? {
            println!("Operation cancelled.");
            return Ok(());
        }
    }

    let manager = PresetManager::new()?;
    let preset = manager
        .load(preset_name)
        .with_context(|| format!("Preset '{preset_name}' not found"))?;

    let builder = SymlinkBuilder::new();
    let created = builder.apply(&preset, target)?;

    // Save link record only with --sync flag
    if sync {
        manager.add_link(preset_name, target)?;
    }

    if !quiet {
        // Print created symlinks
        if !created.is_empty() {
            println!("Created symlinks:");
            for path in &created {
                println!("  + {}", path);
            }
        }

        if sync {
            println!(
                "\nApplied preset '{preset_name}' to '{target_path}'. (tracking enabled, {} symlinks)",
                created.len()
            );
        } else {
            println!(
                "\nApplied preset '{preset_name}' to '{target_path}'. ({} symlinks)",
                created.len()
            );
        }
    }

    Ok(())
}

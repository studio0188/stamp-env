use anyhow::Result;
use std::path::Path;

use crate::preset::manager::PresetManager;
use crate::symlink::builder::SymlinkBuilder;

pub fn run(target: Option<&str>, quiet: bool) -> Result<()> {
    let target_path = target.unwrap_or(".");
    let target = Path::new(target_path);

    let builder = SymlinkBuilder::new();
    let removed = builder.unlink(target)?;

    // Remove link record
    let manager = PresetManager::new()?;
    let preset_name = manager.remove_link(target).ok().flatten();

    if !quiet {
        // Print removed symlinks
        if !removed.is_empty() {
            println!("Removed symlinks:");
            for path in &removed {
                println!("  - {}", path);
            }
        }

        if let Some(preset) = preset_name {
            println!(
                "\nRemoved {} symlinks from '{target_path}'. (preset: {preset})",
                removed.len()
            );
        } else {
            println!(
                "\nRemoved {} symlinks from '{target_path}'.",
                removed.len()
            );
        }
    }

    Ok(())
}

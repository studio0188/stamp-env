use anyhow::Result;
use std::path::Path;

use crate::preset::manager::PresetManager;
use crate::symlink::builder::SymlinkBuilder;

pub fn run(name: &str, patterns: Option<&[String]>, sync: bool, quiet: bool) -> Result<()> {
    let current_dir = Path::new(".");

    let manager = PresetManager::new()?;
    let entries = manager.commit(name, current_dir, patterns)?;

    if !quiet {
        // Print saved files
        let files: Vec<_> = entries.iter().filter(|e| !e.is_dir).collect();
        let dirs: Vec<_> = entries.iter().filter(|e| e.is_dir).collect();

        if !files.is_empty() {
            println!("Saved files:");
            for entry in &files {
                println!("  + {}", entry.path);
            }
        }

        if !dirs.is_empty() {
            println!("Saved directories:");
            for entry in &dirs {
                println!("  + {}/", entry.path);
            }
        }

        if let Some(p) = patterns {
            println!(
                "\nSaved current structure as preset '{name}'. (filter: {:?}, {} files, {} directories)",
                p, files.len(), dirs.len()
            );
        } else {
            println!(
                "\nSaved current structure as preset '{name}'. ({} files, {} directories)",
                files.len(), dirs.len()
            );
        }
    }

    // Sync to linked locations if --sync flag is set
    if sync {
        let targets = manager.get_links(name)?;

        if targets.is_empty() {
            if !quiet {
                println!("No linked locations to sync.");
            }
        } else {
            // Clean up broken links
            let cleaned = manager.cleanup_broken_links()?;
            if !quiet && cleaned > 0 {
                println!("Cleaned up {cleaned} invalid link records.");
            }

            // Reload preset
            let preset = manager.load(name)?;
            let builder = SymlinkBuilder::new();

            let mut synced = 0;
            for target_path in &targets {
                let target = Path::new(target_path);
                if target.exists() {
                    builder.apply(&preset, target)?;
                    synced += 1;
                    if !quiet {
                        println!("  - '{target_path}' synced");
                    }
                }
            }

            if !quiet {
                println!("Synced changes to {synced} locations.");
            }
        }
    }

    Ok(())
}

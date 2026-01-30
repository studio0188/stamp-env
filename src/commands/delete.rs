use anyhow::{Context, Result};
use std::io::{self, Write};
use std::path::Path;

use crate::preset::manager::PresetManager;
use crate::symlink::builder::SymlinkBuilder;

pub fn run(presets: &[String], yes: bool, do_unlink: bool) -> Result<()> {
    let manager = PresetManager::new()?;

    // Validate all presets exist first
    let mut not_found = Vec::new();
    for name in presets {
        if !manager.exists(name) {
            not_found.push(name.clone());
        }
    }

    if !not_found.is_empty() {
        anyhow::bail!(
            "Preset(s) not found: {}",
            not_found.join(", ")
        );
    }

    // Collect linked targets for each preset
    let mut preset_links: Vec<(String, Vec<String>)> = Vec::new();
    for name in presets {
        let links = manager.get_links(name)?;
        preset_links.push((name.clone(), links));
    }

    // Show linked locations warning
    let has_links = preset_links.iter().any(|(_, links)| !links.is_empty());
    if has_links {
        for (name, links) in &preset_links {
            if !links.is_empty() {
                println!("Preset '{}' is linked to:", name);
                for target in links {
                    println!("  - {}", target);
                }
                println!();
            }
        }

        if !do_unlink {
            println!("Warning: These locations will keep their symlinks but lose their preset reference.");
            println!("Use --unlink to also remove symlinks from these locations.\n");
        }
    }

    // Confirmation prompt
    if !yes {
        print!(
            "Delete {} preset(s)? [y/N] ",
            presets.len()
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    // Unlink from all locations if requested
    if do_unlink {
        let builder = SymlinkBuilder::new();
        for (name, links) in &preset_links {
            for target in links {
                let target_path = Path::new(target);
                if target_path.exists() {
                    print!("Unlinking from {}... ", target);
                    io::stdout().flush()?;
                    match builder.unlink(target_path) {
                        Ok(_) => println!("done"),
                        Err(e) => println!("failed: {}", e),
                    }
                } else {
                    println!("Skipping {} (path not found)", target);
                }
            }
            // Remove link records for this preset
            manager.remove_links_for_preset(name)?;
        }
    }

    // Delete presets
    let mut deleted_count = 0;
    for name in presets {
        manager
            .delete(name, !do_unlink) // If we already did unlink, don't remove records again
            .with_context(|| format!("Failed to delete preset '{}'", name))?;
        println!("Deleted preset: {}", name);
        deleted_count += 1;
    }

    println!("\n{} preset(s) deleted.", deleted_count);

    Ok(())
}

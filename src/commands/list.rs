use anyhow::{Context, Result};

use crate::preset::manager::PresetManager;

pub fn run() -> Result<()> {
    let manager = PresetManager::new()?;
    let presets = manager.list()?;

    if presets.is_empty() {
        println!("No saved presets.");
    } else {
        println!("Saved presets:");
        for name in presets {
            println!("  - {name}");
        }
    }

    Ok(())
}

pub fn show(preset_name: &str) -> Result<()> {
    let manager = PresetManager::new()?;
    let preset = manager
        .load(preset_name)
        .with_context(|| format!("Preset '{preset_name}' not found"))?;

    println!("Preset: {preset_name}");
    println!("Files:");
    for entry in &preset.entries {
        println!("  {}", entry.path);
    }

    Ok(())
}

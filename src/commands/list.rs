use anyhow::{Context, Result};

use crate::preset::manager::PresetManager;

pub fn run() -> Result<()> {
    let manager = PresetManager::new()?;
    let presets = manager.list()?;

    if presets.is_empty() {
        println!("저장된 프리셋이 없습니다.");
    } else {
        println!("저장된 프리셋:");
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
        .with_context(|| format!("프리셋 '{preset_name}'을 찾을 수 없습니다"))?;

    println!("프리셋: {preset_name}");
    println!("파일 목록:");
    for entry in &preset.entries {
        println!("  {}", entry.path);
    }

    Ok(())
}

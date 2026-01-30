use anyhow::Result;
use std::path::Path;

use crate::preset::manager::PresetManager;
use crate::symlink::builder::SymlinkBuilder;

pub fn run(target: Option<&str>, quiet: bool) -> Result<()> {
    let target_path = target.unwrap_or(".");
    let target = Path::new(target_path);

    let builder = SymlinkBuilder::new();
    let removed = builder.unlink(target)?;

    // 링크 기록 제거
    let manager = PresetManager::new()?;
    let preset_name = manager.remove_link(target).ok().flatten();

    if !quiet {
        // 제거된 심링크 목록 출력
        if !removed.is_empty() {
            println!("제거된 심링크:");
            for path in &removed {
                println!("  - {}", path);
            }
        }

        if let Some(preset) = preset_name {
            println!(
                "\n'{target_path}'에서 {}개의 심링크를 제거했습니다. (프리셋: {preset})",
                removed.len()
            );
        } else {
            println!(
                "\n'{target_path}'에서 {}개의 심링크를 제거했습니다.",
                removed.len()
            );
        }
    }

    Ok(())
}

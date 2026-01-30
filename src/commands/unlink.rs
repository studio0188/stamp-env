use anyhow::Result;
use std::path::Path;

use crate::preset::manager::PresetManager;
use crate::symlink::builder::SymlinkBuilder;

pub fn run(target: Option<&str>) -> Result<()> {
    let target_path = target.unwrap_or(".");
    let target = Path::new(target_path);

    let builder = SymlinkBuilder::new();
    let count = builder.unlink(target)?;

    // 링크 기록 제거
    let manager = PresetManager::new()?;
    if let Ok(Some(preset_name)) = manager.remove_link(target) {
        println!(
            "'{target_path}'에서 {count}개의 심링크를 제거했습니다. (프리셋: {preset_name})"
        );
    } else {
        println!("'{target_path}'에서 {count}개의 심링크를 제거했습니다.");
    }

    Ok(())
}

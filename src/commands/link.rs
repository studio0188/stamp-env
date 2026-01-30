use anyhow::{Context, Result};
use std::io::{self, Write};
use std::path::Path;

use crate::preset::manager::PresetManager;
use crate::symlink::builder::SymlinkBuilder;

/// 디렉토리가 비어있는지 확인
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

/// 사용자에게 확인 요청
fn confirm(message: &str) -> Result<bool> {
    print!("{} [y/N]: ", message);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_lowercase();
    Ok(input == "y" || input == "yes")
}

pub fn run(preset_name: &str, target: Option<&str>, skip_confirm: bool) -> Result<()> {
    let target_path = target.unwrap_or(".");
    let target = Path::new(target_path);

    // 디렉토리가 비어있지 않으면 확인 요청
    if !skip_confirm && !is_directory_empty(target)? {
        let message = format!(
            "대상 디렉토리 '{}'가 비어있지 않습니다. 계속하시겠습니까?",
            target_path
        );
        if !confirm(&message)? {
            println!("작업이 취소되었습니다.");
            return Ok(());
        }
    }

    let manager = PresetManager::new()?;
    let preset = manager
        .load(preset_name)
        .with_context(|| format!("프리셋 '{preset_name}'을 찾을 수 없습니다"))?;

    let builder = SymlinkBuilder::new();
    builder.apply(&preset, target)?;

    // 링크 기록 저장
    manager.add_link(preset_name, target)?;

    println!("프리셋 '{preset_name}'을 '{target_path}'에 적용했습니다.");
    Ok(())
}

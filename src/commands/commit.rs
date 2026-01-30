use anyhow::Result;
use std::path::Path;

use crate::preset::manager::PresetManager;
use crate::symlink::builder::SymlinkBuilder;

pub fn run(name: &str, patterns: Option<&[String]>, sync: bool) -> Result<()> {
    let current_dir = Path::new(".");

    let manager = PresetManager::new()?;
    manager.commit(name, current_dir, patterns)?;

    if let Some(p) = patterns {
        println!(
            "현재 구조를 프리셋 '{name}'으로 저장했습니다. (필터: {:?})",
            p
        );
    } else {
        println!("현재 구조를 프리셋 '{name}'으로 저장했습니다.");
    }

    // --sync 플래그가 있으면 link된 위치들에 동기화
    if sync {
        let targets = manager.get_links(name)?;

        if targets.is_empty() {
            println!("동기화할 link된 위치가 없습니다.");
        } else {
            // 깨진 링크 정리
            let cleaned = manager.cleanup_broken_links()?;
            if cleaned > 0 {
                println!("유효하지 않은 {cleaned}개의 링크 기록을 정리했습니다.");
            }

            // 프리셋 다시 로드
            let preset = manager.load(name)?;
            let builder = SymlinkBuilder::new();

            let mut synced = 0;
            for target_path in &targets {
                let target = Path::new(target_path);
                if target.exists() {
                    builder.apply(&preset, target)?;
                    synced += 1;
                    println!("  - '{target_path}' 동기화 완료");
                }
            }

            println!("{synced}개의 위치에 변경사항을 동기화했습니다.");
        }
    }

    Ok(())
}

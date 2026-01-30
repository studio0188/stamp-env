use anyhow::Result;
use std::path::Path;

use crate::preset::manager::PresetManager;
use crate::symlink::builder::SymlinkBuilder;

pub fn run(name: &str, patterns: Option<&[String]>, sync: bool, quiet: bool) -> Result<()> {
    let current_dir = Path::new(".");

    let manager = PresetManager::new()?;
    let entries = manager.commit(name, current_dir, patterns)?;

    if !quiet {
        // 저장된 파일 목록 출력
        let files: Vec<_> = entries.iter().filter(|e| !e.is_dir).collect();
        let dirs: Vec<_> = entries.iter().filter(|e| e.is_dir).collect();

        if !files.is_empty() {
            println!("저장된 파일:");
            for entry in &files {
                println!("  + {}", entry.path);
            }
        }

        if !dirs.is_empty() {
            println!("저장된 디렉토리:");
            for entry in &dirs {
                println!("  + {}/", entry.path);
            }
        }

        if let Some(p) = patterns {
            println!(
                "\n현재 구조를 프리셋 '{name}'으로 저장했습니다. (필터: {:?}, {}개 파일, {}개 디렉토리)",
                p, files.len(), dirs.len()
            );
        } else {
            println!(
                "\n현재 구조를 프리셋 '{name}'으로 저장했습니다. ({}개 파일, {}개 디렉토리)",
                files.len(), dirs.len()
            );
        }
    }

    // --sync 플래그가 있으면 link된 위치들에 동기화
    if sync {
        let targets = manager.get_links(name)?;

        if targets.is_empty() {
            if !quiet {
                println!("동기화할 link된 위치가 없습니다.");
            }
        } else {
            // 깨진 링크 정리
            let cleaned = manager.cleanup_broken_links()?;
            if !quiet && cleaned > 0 {
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
                    if !quiet {
                        println!("  - '{target_path}' 동기화 완료");
                    }
                }
            }

            if !quiet {
                println!("{synced}개의 위치에 변경사항을 동기화했습니다.");
            }
        }
    }

    Ok(())
}

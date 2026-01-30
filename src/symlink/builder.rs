use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::preset::Preset;

pub struct SymlinkBuilder;

impl SymlinkBuilder {
    pub fn new() -> Self {
        Self
    }

    /// 프리셋을 대상 경로에 심링크로 적용
    pub fn apply(&self, preset: &Preset, target: &Path) -> Result<()> {
        let source_base = Path::new(&preset.source);

        for entry in &preset.entries {
            let source = source_base.join(&entry.path);
            let dest = target.join(&entry.path);

            if entry.is_dir {
                fs::create_dir_all(&dest)
                    .with_context(|| format!("디렉토리 생성 실패: {}", dest.display()))?;
            } else {
                // 부모 디렉토리 생성
                if let Some(parent) = dest.parent() {
                    fs::create_dir_all(parent)?;
                }

                // 기존 파일/심링크 확인
                if dest.exists() || dest.symlink_metadata().is_ok() {
                    fs::remove_file(&dest)
                        .with_context(|| format!("기존 파일 제거 실패: {}", dest.display()))?;
                }

                // 심링크 생성
                #[cfg(unix)]
                std::os::unix::fs::symlink(&source, &dest)
                    .with_context(|| format!("심링크 생성 실패: {} -> {}", dest.display(), source.display()))?;

                #[cfg(windows)]
                std::os::windows::fs::symlink_file(&source, &dest)
                    .with_context(|| format!("심링크 생성 실패: {} -> {}", dest.display(), source.display()))?;
            }
        }

        Ok(())
    }

    /// 대상 경로에서 심링크 제거
    pub fn unlink(&self, target: &Path) -> Result<usize> {
        let mut count = 0;

        for entry in walkdir::WalkDir::new(target)
            .min_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            // 심링크인지 확인
            if let Ok(metadata) = path.symlink_metadata() {
                if metadata.file_type().is_symlink() {
                    fs::remove_file(path)
                        .with_context(|| format!("심링크 제거 실패: {}", path.display()))?;
                    count += 1;
                }
            }
        }

        Ok(count)
    }
}

impl Default for SymlinkBuilder {
    fn default() -> Self {
        Self::new()
    }
}

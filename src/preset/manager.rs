use anyhow::{Context, Result};
use glob::Pattern;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use super::{Preset, PresetEntry};

pub struct PresetManager {
    presets_dir: PathBuf,
}

impl PresetManager {
    pub fn new() -> Result<Self> {
        let presets_dir = dirs::config_dir()
            .context("설정 디렉토리를 찾을 수 없습니다")?
            .join("stamp.env")
            .join("presets");

        fs::create_dir_all(&presets_dir)
            .context("프리셋 디렉토리를 생성할 수 없습니다")?;

        Ok(Self { presets_dir })
    }

    /// 프리셋 목록 조회
    pub fn list(&self) -> Result<Vec<String>> {
        let mut presets = Vec::new();

        if self.presets_dir.exists() {
            for entry in fs::read_dir(&self.presets_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "toml") {
                    if let Some(name) = path.file_stem() {
                        presets.push(name.to_string_lossy().to_string());
                    }
                }
            }
        }

        presets.sort();
        Ok(presets)
    }

    /// 프리셋 로드
    pub fn load(&self, name: &str) -> Result<Preset> {
        let path = self.presets_dir.join(format!("{name}.toml"));
        let content = fs::read_to_string(&path)
            .with_context(|| format!("프리셋 파일을 읽을 수 없습니다: {}", path.display()))?;

        let preset: Preset = toml::from_str(&content)
            .with_context(|| format!("프리셋 파일을 파싱할 수 없습니다: {}", path.display()))?;

        Ok(preset)
    }

    /// 현재 디렉토리 구조를 프리셋으로 저장
    pub fn commit(&self, name: &str, source: &Path, patterns: Option<&[String]>) -> Result<()> {
        let source_abs = source
            .canonicalize()
            .context("소스 경로를 확인할 수 없습니다")?;

        // Glob 패턴 컴파일
        let compiled_patterns: Option<Vec<Pattern>> = patterns.map(|p| {
            p.iter()
                .filter_map(|pat| Pattern::new(pat).ok())
                .collect()
        });

        let mut entries = Vec::new();

        for entry in WalkDir::new(&source_abs)
            .min_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            let relative = path
                .strip_prefix(&source_abs)
                .context("상대 경로를 계산할 수 없습니다")?;

            let relative_str = relative.to_string_lossy();

            // 패턴 필터링: 패턴이 지정된 경우, 하나 이상 매칭되어야 포함
            let should_include = match &compiled_patterns {
                Some(pats) if !pats.is_empty() => {
                    pats.iter().any(|pat| pat.matches(&relative_str))
                }
                _ => true,
            };

            if should_include {
                entries.push(PresetEntry {
                    path: relative_str.to_string(),
                    is_dir: path.is_dir(),
                });
            }
        }

        let preset = Preset {
            name: name.to_string(),
            source: source_abs.to_string_lossy().to_string(),
            entries,
        };

        let content = toml::to_string_pretty(&preset)
            .context("프리셋을 직렬화할 수 없습니다")?;

        let path = self.presets_dir.join(format!("{name}.toml"));
        fs::write(&path, content)
            .with_context(|| format!("프리셋 파일을 저장할 수 없습니다: {}", path.display()))?;

        Ok(())
    }
}

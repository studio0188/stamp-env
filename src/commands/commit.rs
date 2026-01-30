use anyhow::Result;
use std::path::Path;

use crate::preset::manager::PresetManager;

pub fn run(name: &str, patterns: Option<&[String]>) -> Result<()> {
    let current_dir = Path::new(".");

    let manager = PresetManager::new()?;
    manager.commit(name, current_dir, patterns)?;

    if let Some(p) = patterns {
        println!("현재 구조를 프리셋 '{name}'으로 저장했습니다. (필터: {:?})", p);
    } else {
        println!("현재 구조를 프리셋 '{name}'으로 저장했습니다.");
    }
    Ok(())
}

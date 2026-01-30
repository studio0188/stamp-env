use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::preset::Preset;

pub struct SymlinkBuilder;

impl SymlinkBuilder {
    pub fn new() -> Self {
        Self
    }

    /// Apply a preset to the target path via symlinks
    /// Returns a list of created symlink paths
    pub fn apply(&self, preset: &Preset, target: &Path) -> Result<Vec<String>> {
        let source_base = Path::new(&preset.source);
        let mut created = Vec::new();

        for entry in &preset.entries {
            let source = source_base.join(&entry.path);
            let dest = target.join(&entry.path);

            if entry.is_dir {
                fs::create_dir_all(&dest)
                    .with_context(|| format!("Failed to create directory: {}", dest.display()))?;
            } else {
                // Create parent directory
                if let Some(parent) = dest.parent() {
                    fs::create_dir_all(parent)?;
                }

                // Check for existing file/symlink
                if dest.exists() || dest.symlink_metadata().is_ok() {
                    fs::remove_file(&dest)
                        .with_context(|| format!("Failed to remove existing file: {}", dest.display()))?;
                }

                // Create symlink
                #[cfg(unix)]
                std::os::unix::fs::symlink(&source, &dest)
                    .with_context(|| format!("Failed to create symlink: {} -> {}", dest.display(), source.display()))?;

                #[cfg(windows)]
                std::os::windows::fs::symlink_file(&source, &dest)
                    .with_context(|| format!("Failed to create symlink: {} -> {}", dest.display(), source.display()))?;

                created.push(dest.to_string_lossy().to_string());
            }
        }

        Ok(created)
    }

    /// Remove symlinks from the target path
    /// Returns a list of removed symlink paths
    pub fn unlink(&self, target: &Path) -> Result<Vec<String>> {
        let mut removed = Vec::new();

        for entry in walkdir::WalkDir::new(target)
            .min_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            // Check if it's a symlink
            if let Ok(metadata) = path.symlink_metadata() {
                if metadata.file_type().is_symlink() {
                    let path_str = path.to_string_lossy().to_string();
                    fs::remove_file(path)
                        .with_context(|| format!("Failed to remove symlink: {}", path.display()))?;
                    removed.push(path_str);
                }
            }
        }

        Ok(removed)
    }
}

impl Default for SymlinkBuilder {
    fn default() -> Self {
        Self::new()
    }
}

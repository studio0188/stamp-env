use anyhow::{Context, Result};
use chrono::Utc;
use glob::Pattern;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use super::{LinkRecord, LinksRegistry, Preset, PresetEntry};

pub struct PresetManager {
    presets_dir: PathBuf,
}

impl PresetManager {
    pub fn new() -> Result<Self> {
        let presets_dir = dirs::config_dir()
            .context("Could not find config directory")?
            .join("stamp.env")
            .join("presets");

        fs::create_dir_all(&presets_dir)
            .context("Could not create presets directory")?;

        Ok(Self { presets_dir })
    }

    /// List all presets
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

    /// Load a preset
    pub fn load(&self, name: &str) -> Result<Preset> {
        let path = self.presets_dir.join(format!("{name}.toml"));
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Could not read preset file: {}", path.display()))?;

        let preset: Preset = toml::from_str(&content)
            .with_context(|| format!("Could not parse preset file: {}", path.display()))?;

        Ok(preset)
    }

    /// Save current directory structure as a preset
    /// Returns the list of saved entries
    pub fn commit(&self, name: &str, source: &Path, patterns: Option<&[String]>) -> Result<Vec<PresetEntry>> {
        let source_abs = source
            .canonicalize()
            .context("Could not resolve source path")?;

        // Compile glob patterns
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
                .context("Could not calculate relative path")?;

            let relative_str = relative.to_string_lossy();

            // Pattern filtering: if patterns are specified, include only if at least one matches
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
            entries: entries.clone(),
        };

        let content = toml::to_string_pretty(&preset)
            .context("Could not serialize preset")?;

        let path = self.presets_dir.join(format!("{name}.toml"));
        fs::write(&path, content)
            .with_context(|| format!("Could not save preset file: {}", path.display()))?;

        Ok(entries)
    }

    /// Path to links.toml file
    fn links_file(&self) -> PathBuf {
        self.presets_dir.parent().unwrap().join("links.toml")
    }

    /// Load links registry
    fn load_links_registry(&self) -> Result<LinksRegistry> {
        let path = self.links_file();
        if !path.exists() {
            return Ok(LinksRegistry::default());
        }

        let content = fs::read_to_string(&path)
            .with_context(|| format!("Could not read links registry: {}", path.display()))?;

        let registry: LinksRegistry = toml::from_str(&content)
            .with_context(|| format!("Could not parse links registry: {}", path.display()))?;

        Ok(registry)
    }

    /// Save links registry
    fn save_links_registry(&self, registry: &LinksRegistry) -> Result<()> {
        let path = self.links_file();
        let content = toml::to_string_pretty(registry)
            .context("Could not serialize links registry")?;

        fs::write(&path, content)
            .with_context(|| format!("Could not save links registry: {}", path.display()))?;

        Ok(())
    }

    /// Add a link record
    pub fn add_link(&self, preset_name: &str, target: &Path) -> Result<()> {
        let target_abs = target
            .canonicalize()
            .with_context(|| format!("Could not resolve target path: {}", target.display()))?;
        let target_str = target_abs.to_string_lossy().to_string();

        let mut registry = self.load_links_registry()?;

        // Update if same preset + target combination already exists
        if let Some(existing) = registry
            .links
            .iter_mut()
            .find(|r| r.preset == preset_name && r.target == target_str)
        {
            existing.linked_at = Utc::now().to_rfc3339();
        } else {
            registry.links.push(LinkRecord {
                preset: preset_name.to_string(),
                target: target_str,
                linked_at: Utc::now().to_rfc3339(),
            });
        }

        self.save_links_registry(&registry)?;
        Ok(())
    }

    /// Remove link record for a specific target path
    pub fn remove_link(&self, target: &Path) -> Result<Option<String>> {
        let target_abs = target
            .canonicalize()
            .with_context(|| format!("Could not resolve target path: {}", target.display()))?;
        let target_str = target_abs.to_string_lossy().to_string();

        let mut registry = self.load_links_registry()?;

        // Find link to remove
        let removed_preset = registry
            .links
            .iter()
            .find(|r| r.target == target_str)
            .map(|r| r.preset.clone());

        registry.links.retain(|r| r.target != target_str);

        self.save_links_registry(&registry)?;
        Ok(removed_preset)
    }

    /// Get all linked target paths for a specific preset
    pub fn get_links(&self, preset_name: &str) -> Result<Vec<String>> {
        let registry = self.load_links_registry()?;

        let targets: Vec<String> = registry
            .links
            .iter()
            .filter(|r| r.preset == preset_name)
            .map(|r| r.target.clone())
            .collect();

        Ok(targets)
    }

    /// Clean up non-existent target paths
    pub fn cleanup_broken_links(&self) -> Result<usize> {
        let mut registry = self.load_links_registry()?;
        let original_count = registry.links.len();

        registry.links.retain(|r| Path::new(&r.target).exists());

        let removed = original_count - registry.links.len();
        if removed > 0 {
            self.save_links_registry(&registry)?;
        }

        Ok(removed)
    }

    /// Check if a preset exists
    pub fn exists(&self, name: &str) -> bool {
        let path = self.presets_dir.join(format!("{name}.toml"));
        path.exists()
    }

    /// Delete a preset and optionally remove its link records
    pub fn delete(&self, name: &str, remove_link_records: bool) -> Result<()> {
        let path = self.presets_dir.join(format!("{name}.toml"));
        if !path.exists() {
            anyhow::bail!("Preset '{name}' not found");
        }
        fs::remove_file(&path)
            .with_context(|| format!("Could not delete preset: {}", path.display()))?;

        // Remove link records for this preset if requested
        if remove_link_records {
            self.remove_links_for_preset(name)?;
        }
        Ok(())
    }

    /// Remove all link records for a specific preset
    pub fn remove_links_for_preset(&self, preset_name: &str) -> Result<usize> {
        let mut registry = self.load_links_registry()?;
        let original_count = registry.links.len();
        registry.links.retain(|r| r.preset != preset_name);
        let removed = original_count - registry.links.len();
        if removed > 0 {
            self.save_links_registry(&registry)?;
        }
        Ok(removed)
    }
}

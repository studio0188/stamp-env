pub mod manager;

use serde::{Deserialize, Serialize};

/// Individual item in a preset (file or directory)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetEntry {
    /// Relative path
    pub path: String,
    /// Whether it's a directory
    pub is_dir: bool,
}

/// Preset definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    /// Preset name
    pub name: String,
    /// Source path (absolute path)
    pub source: String,
    /// List of included files/directories
    pub entries: Vec<PresetEntry>,
}

/// Individual link record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkRecord {
    /// Preset name
    pub preset: String,
    /// Linked target path (absolute path)
    pub target: String,
    /// Link creation time (ISO 8601)
    pub linked_at: String,
}

/// Registry managing all link records
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinksRegistry {
    /// List of link records
    pub links: Vec<LinkRecord>,
}

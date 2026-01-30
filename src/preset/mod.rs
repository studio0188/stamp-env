pub mod manager;

use serde::{Deserialize, Serialize};

/// 프리셋 내 개별 항목 (파일 또는 디렉토리)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetEntry {
    /// 상대 경로
    pub path: String,
    /// 디렉토리 여부
    pub is_dir: bool,
}

/// 프리셋 정의
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    /// 프리셋 이름
    pub name: String,
    /// 원본 경로 (절대 경로)
    pub source: String,
    /// 포함된 파일/디렉토리 목록
    pub entries: Vec<PresetEntry>,
}

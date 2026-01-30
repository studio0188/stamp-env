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

/// 개별 link 기록
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkRecord {
    /// 프리셋 이름
    pub preset: String,
    /// link된 대상 경로 (절대 경로)
    pub target: String,
    /// link 생성 시간 (ISO 8601)
    pub linked_at: String,
}

/// 모든 link 기록을 관리하는 레지스트리
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinksRegistry {
    /// link 기록 목록
    pub links: Vec<LinkRecord>,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileCategory {
    Photos,
    Videos,
    Audio,
    Documents,
    Other,
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: std::path::PathBuf,
    pub category: FileCategory,
    pub size: u64,
    pub modified: chrono::DateTime<chrono::Utc>,
}

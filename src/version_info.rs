use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub short_version: String,
    pub dsym: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    pub download: String,
}

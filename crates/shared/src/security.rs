//! Security scanning, language stats and template/notice helpers.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityVulnerability {
    pub id: String,
    pub severity: String, // "CRITICAL", "HIGH", "MEDIUM", "LOW"
    pub title: String,
    pub description: String,
    pub file_path: String,
    pub line_no: u64,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityScanReport {
    pub id: String,
    pub repo_owner: String,
    pub repo_name: String,
    pub score: u8, // 0 to 100
    pub vulnerabilities: Vec<SecurityVulnerability>,
    pub scanned_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LanguageStat {
    pub language: String,
    pub percentage: u8,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LicenseTemplate {
    pub key: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GitignoreTemplate {
    pub name: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemNotice {
    pub id: u64,
    pub type_: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TwoFactor {
    pub enabled: bool,
    pub method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmailAddress {
    pub email: String,
    pub verified: bool,
    pub primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contribution {
    pub date: String,
    pub count: u64,
}

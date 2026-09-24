//! Template and metadata helpers (licenses, gitignores, language stats).

use axum::extract::{Json, Path};
use shared::{GitignoreTemplate, LanguageStat, LicenseTemplate};

// Miscellaneous Handlers

pub async fn list_licenses() -> Json<Vec<LicenseTemplate>> {
    vec![LicenseTemplate {
        key: "mit".to_string(),
        name: "MIT License".to_string(),
        url: "http://...".to_string(),
    }]
    .into()
}

pub async fn list_gitignores() -> Json<Vec<GitignoreTemplate>> {
    vec![GitignoreTemplate {
        name: "Rust".to_string(),
        source: "target/".to_string(),
    }]
    .into()
}

pub async fn get_repo_languages(
    Path((_owner, _repo)): Path<(String, String)>,
) -> Json<Vec<LanguageStat>> {
    vec![LanguageStat {
        language: "Rust".to_string(),
        percentage: 100,
        color: "#dea584".to_string(),
    }]
    .into()
}

//! Releases, release assets and the package registry.

use crate::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Release {
    pub id: u64,
    pub repo_id: u64,
    pub tag_name: String,
    pub name: String,
    pub body: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub author: User,
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReleaseAsset {
    pub id: u64,
    pub name: String,
    pub size: u64,
    pub download_url: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateReleaseOption {
    pub tag_name: String,
    pub name: String,
    pub body: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateReleaseOption {
    pub tag_name: Option<String>,
    pub name: Option<String>,
    pub body: Option<String>,
    pub draft: Option<bool>,
    pub prerelease: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Package {
    pub id: u64,
    pub owner: String,
    pub name: String,
    pub version: String,
    pub package_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreatePackageOption {
    pub name: String,
    pub version: String,
    pub package_type: String, // "npm", "maven", "cargo", etc.
}

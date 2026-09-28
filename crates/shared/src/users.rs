//! User accounts, credentials, keys, OAuth2 apps and notifications.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: Option<String>,
}

impl User {
    pub fn new(id: u64, username: String, email: Option<String>) -> Self {
        Self {
            id,
            username,
            email,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GpgKey {
    pub id: u64,
    pub key_id: String,
    pub primary_key_id: String,
    pub public_key: String,
    pub emails: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateGpgKeyOption {
    pub armored_public_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoginOption {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RegisterOption {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserSettingsOption {
    pub full_name: Option<String>,
    pub website: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PublicKey {
    pub id: u64,
    pub title: String,
    pub key: String,
    pub fingerprint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateKeyOption {
    pub title: String,
    pub key: String,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct OAuth2Application {
    pub id: u64,
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uris: Vec<String>,
}

impl fmt::Debug for OAuth2Application {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OAuth2Application")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("client_id", &self.client_id)
            .field("client_secret", &"***REDACTED***")
            .field("redirect_uris", &self.redirect_uris)
            .finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateOAuth2AppOption {
    pub name: String,
    pub redirect_uris: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdminUserEditOption {
    pub email: Option<String>,
    pub password: Option<String>,
    pub active: Option<bool>,
    pub admin: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OAuth2Provider {
    pub name: String,
    pub display_name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Notification {
    pub id: u64,
    pub subject: String,
    pub unread: bool,
    pub updated_at: String,
}

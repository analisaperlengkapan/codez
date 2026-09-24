use crate::state::AppState;
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::{Duration, Utc};
use serde::Serialize;
use shared::{
    Activity, Branch, CodeSearchResult, Collaborator, Comment, Commit, CommitStatus,
    CreateBranchOption, CreateCommentOption, CreateHookOption, CreateIssueOption, CreateKeyOption,
    CreateLabelOption, CreateMilestoneOption, CreateProtectedBranchOption, CreatePullRequestOption,
    CreateReactionOption, CreateRepoOption, CreateReviewOption, CreateSecretOption,
    CreateStatusOption, CreateWikiPageOption, DeployKey, DiffFile, DiffLine, FileEntry, Issue,
    IssueEvent, IssueFilterOptions, Label, LfsLock, LfsObject, MergePullRequestOption,
    MigrateRepoOption, Milestone, MilestoneStats, Notification, PaginationOptions, ProtectedBranch,
    PullRequest, PullRequestEvent, PushEvent, Reaction, RepoPulseStats, RepoSearchOptions,
    RepoSettingsOption, RepoTopicOptions, RepoUserStatus, Repository, Review, ReviewRequest,
    Secret, SecurityScanReport, SecurityVulnerability, Tag, Topic, TransferRepoOption,
    UpdateCommentOption, UpdateFileOption, UpdateIssueOption, UpdateLabelOption,
    UpdateMilestoneOption, UpdatePullRequestOption, User, Webhook, WebhookDelivery, WikiPage,
};
use url::Url;

#[derive(serde::Deserialize)]
pub struct GetContentQuery {
    #[serde(rename = "ref")]
    pub ref_name: Option<String>,
}

fn dispatch_hooks<T: Serialize + Send + Sync + 'static + Clone>(
    state: &AppState,
    repo_id: u64,
    event: &str,
    payload: T,
) {
    let hooks = state.hooks.read().unwrap_or_else(|e| e.into_inner());
    let relevant_hooks: Vec<Webhook> = hooks
        .iter()
        .filter(|h| h.repo_id == repo_id && h.active && h.events.contains(&event.to_string()))
        .cloned()
        .collect();

    if relevant_hooks.is_empty() {
        return;
    }

    let state_clone = state.clone();
    let event_string = event.to_string();

    tokio::spawn(async move {
        for hook in relevant_hooks {
            // SSRF Protection with DNS Pinning via reqwest::resolve
            let validated_target = validate_and_resolve_webhook_url(&hook.url).await;

            let (status_str, status_code) = if let Some((host, _port, safe_addr)) = validated_target
            {
                // We must build a new client for each hook to apply the specific DNS resolution override
                // while keeping the original URL for correct TLS validation (SNI).
                let client = reqwest::Client::builder()
                    .timeout(std::time::Duration::from_secs(10))
                    .resolve(&host, safe_addr)
                    .build()
                    .unwrap_or_default();

                let response = client
                    .post(&hook.url)
                    .header("X-Codeza-Event", &event_string)
                    .header("X-Codeza-Delivery", uuid::Uuid::new_v4().to_string())
                    .json(&payload)
                    .send()
                    .await;

                match response {
                    Ok(resp) => {
                        let s = resp.status();
                        (
                            if s.is_success() { "success" } else { "failed" }.to_string(),
                            s.as_u16(),
                        )
                    }
                    Err(_) => ("failed".to_string(), 0),
                }
            } else {
                ("failed (blocked)".to_string(), 0)
            };

            let mut deliveries = state_clone
                .webhook_deliveries
                .write()
                .unwrap_or_else(|e| e.into_inner());
            let delivery_id = (deliveries.len() as u64) + 1;
            deliveries.push(WebhookDelivery {
                id: delivery_id,
                hook_id: hook.id,
                event: event_string.clone(),
                status: status_str,
                request_url: hook.url.clone(),
                response_status: status_code,
                delivered_at: "now".to_string(),
            });
        }
    });
}

#[derive(serde::Deserialize)]
pub struct LfsLockRequest {
    pub path: String,
}

fn is_private_ipv4(ipv4: std::net::Ipv4Addr) -> bool {
    // Check RFC 1918 and Link-Local
    // 10.0.0.0/8
    (ipv4.octets()[0] == 10) ||
    // 172.16.0.0/12
    (ipv4.octets()[0] == 172 && (16..=31).contains(&ipv4.octets()[1])) ||
    // 192.168.0.0/16
    (ipv4.octets()[0] == 192 && ipv4.octets()[1] == 168) ||
    // 169.254.0.0/16 (Link Local)
    (ipv4.octets()[0] == 169 && ipv4.octets()[1] == 254)
}

// Returns Some((host, port, safe_addr)) if safe, None otherwise.

async fn validate_and_resolve_webhook_url(
    url: &str,
) -> Option<(String, u16, std::net::SocketAddr)> {
    if let Ok(parsed_url) = Url::parse(url) {
        if parsed_url.scheme() != "http" && parsed_url.scheme() != "https" {
            return None;
        }
        if let Some(host) = parsed_url.host_str() {
            let port = parsed_url
                .port()
                .unwrap_or(if parsed_url.scheme() == "https" {
                    443
                } else {
                    80
                });

            // Format address correctly for IPv6 (must be bracketed if it contains colons)
            let addr_str = if host.contains(':') {
                format!("[{}]:{}", host, port)
            } else {
                format!("{}:{}", host, port)
            };

            // Resolve hostname asynchronously
            if let Ok(mut addrs) = tokio::net::lookup_host(addr_str).await {
                // Check first resolved address
                if let Some(addr) = addrs.next() {
                    let ip = addr.ip();
                    if ip.is_loopback() || ip.is_unspecified() {
                        return None;
                    }
                    let is_private = match ip {
                        std::net::IpAddr::V4(ipv4) => is_private_ipv4(ipv4),
                        std::net::IpAddr::V6(ipv6) => {
                            if let Some(ipv4) = ipv6.to_ipv4_mapped() {
                                is_private_ipv4(ipv4)
                            } else {
                                // Unique Local (fc00::/7)
                                ((ipv6.segments()[0] & 0xfe00) == 0xfc00) ||
                                // Link Local (fe80::/10)
                                ((ipv6.segments()[0] & 0xffc0) == 0xfe80)
                            }
                        }
                    };

                    if is_private {
                        return None;
                    }

                    return Some((host.to_string(), port, addr));
                }
            }
        }
    }
    None
}

fn process_mentions(text: &str) -> Vec<String> {
    let mut mentions = std::collections::HashSet::new();
    for word in text.split_whitespace() {
        if word.starts_with('@') {
            let username = word
                .trim_start_matches('@')
                .trim_end_matches(|c: char| !c.is_alphanumeric() && c != '_' && c != '-'); // common username chars
            if !username.is_empty() {
                mentions.insert(username.to_string());
            }
        }
    }
    mentions.into_iter().collect()
}

// Helper function to extract closing keywords (e.g., "Closes #1")

fn process_closers(text: &str) -> Vec<u64> {
    let keywords = [
        "close", "closes", "closed", "fix", "fixes", "fixed", "resolve", "resolves", "resolved",
    ];
    let mut issue_ids = std::collections::HashSet::new();
    let words: Vec<&str> = text.split_whitespace().collect();

    for (i, word) in words.iter().enumerate() {
        let lower = word.to_lowercase();
        // Allow punctuation after keyword (e.g. "Fixes: #1")
        let clean_keyword = lower.trim_end_matches(|c: char| !c.is_alphanumeric());

        if keywords.contains(&clean_keyword) && i + 1 < words.len() {
            let next = words[i + 1];
            if next.starts_with('#') {
                let id_str = next
                    .trim_start_matches('#')
                    .trim_end_matches(|c: char| !c.is_ascii_digit());
                if let Ok(id) = id_str.parse::<u64>() {
                    issue_ids.insert(id);
                }
            }
        }
    }
    issue_ids.into_iter().collect()
}

mod comments;
mod contents;
mod issues;
mod pulls;
mod pulse;
mod repos;
mod webhooks;
mod wiki;

pub use comments::*;
pub use contents::*;
pub use issues::*;
pub use pulls::*;
pub use pulse::*;
pub use repos::*;
pub use webhooks::*;
pub use wiki::*;

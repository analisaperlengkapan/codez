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
            let (status_str, status_code) =
                deliver_webhook(&hook.url, &event_string, &payload).await;

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

/// Build the SSRF-pinned HTTP client for a validated webhook destination.
///
/// Redirects are disabled: only the first destination is validated and pinned,
/// so following a redirect would let a public endpoint bounce the request to an
/// internal address (CWE-918). The URL stays intact so TLS validation (SNI)
/// still targets the real host.
fn pinned_client(host: &str, safe_addr: std::net::SocketAddr) -> reqwest::Result<reqwest::Client> {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .redirect(reqwest::redirect::Policy::none())
        .resolve(host, safe_addr)
        .build()
}

/// Perform one webhook request, returning `(status_label, response_status)`.
///
/// The destination host is resolved and validated, then pinned so the
/// connection cannot be re-pointed by a rebinding DNS answer. A blocked or
/// unbuildable client fails closed as `failed (blocked)`.
async fn deliver_webhook<T: Serialize>(url: &str, event: &str, payload: &T) -> (String, u16) {
    let Some((host, _port, safe_addr)) = validate_and_resolve_webhook_url(url).await else {
        return ("failed (blocked)".to_string(), 0);
    };

    // Fail closed: never fall back to an unpinned default client.
    let Ok(client) = pinned_client(&host, safe_addr) else {
        return ("failed (blocked)".to_string(), 0);
    };

    match client
        .post(url)
        .header("X-Codeza-Event", event)
        .header("X-Codeza-Delivery", uuid::Uuid::new_v4().to_string())
        .json(payload)
        .send()
        .await
    {
        Ok(resp) => {
            let s = resp.status();
            (
                if s.is_success() { "success" } else { "failed" }.to_string(),
                s.as_u16(),
            )
        }
        Err(_) => ("failed".to_string(), 0),
    }
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

#[cfg(test)]
mod tests {
    use super::{is_private_ipv4, pinned_client, process_closers, process_mentions};
    use std::net::{Ipv4Addr, SocketAddr};

    #[test]
    fn mentions_are_extracted_and_deduplicated() {
        let mentions = process_mentions("thanks @alice and @bob, ping @alice again");
        assert!(mentions.contains(&"alice".to_string()));
        assert!(mentions.contains(&"bob".to_string()));
        assert_eq!(mentions.len(), 2);
    }

    #[test]
    fn mention_trailing_punctuation_is_stripped() {
        let mentions = process_mentions("cc @carol, @dave!");
        assert!(mentions.contains(&"carol".to_string()));
        assert!(mentions.contains(&"dave".to_string()));
    }

    #[test]
    fn bare_at_sign_is_ignored() {
        assert!(process_mentions("email me @ home").is_empty());
    }

    #[test]
    fn a_word_without_at_is_not_a_mention() {
        assert!(process_mentions("alice bob").is_empty());
    }

    #[test]
    fn closing_keywords_capture_issue_numbers() {
        let closed = process_closers("This fixes #12 and closes #34");
        assert!(closed.contains(&12));
        assert!(closed.contains(&34));
        assert_eq!(closed.len(), 2);
    }

    #[test]
    fn closing_keyword_is_case_insensitive_and_punctuation_tolerant() {
        let closed = process_closers("Fixes: #7\nResolved #9.");
        assert!(closed.contains(&7));
        assert!(closed.contains(&9));
    }

    #[test]
    fn non_closing_keywords_and_missing_numbers_are_ignored() {
        assert!(process_closers("related to #5 but not closing it").is_empty());
        assert!(process_closers("closes the door").is_empty());
    }

    #[test]
    fn closes_returns_unique_numbers() {
        let closed = process_closers("closes #3 fixes #3");
        assert_eq!(closed, vec![3]);
    }

    #[test]
    fn rfc1918_and_link_local_ranges_are_private() {
        for ip in [
            "10.0.0.1",
            "10.255.255.255",
            "172.16.0.1",
            "172.31.255.254",
            "192.168.1.1",
            "169.254.10.10",
        ] {
            let addr: Ipv4Addr = ip.parse().unwrap();
            assert!(is_private_ipv4(addr), "{ip} should be private");
        }
    }

    #[test]
    fn public_addresses_are_not_private() {
        for ip in [
            "8.8.8.8",
            "1.1.1.1",
            "172.32.0.1",
            "172.15.0.1",
            "192.169.0.1",
        ] {
            let addr: Ipv4Addr = ip.parse().unwrap();
            assert!(!is_private_ipv4(addr), "{ip} should be public");
        }
    }

    /// A `3xx` from a validated endpoint must not be chased: only the first
    /// (validated + pinned) destination is trusted, so a redirect could bounce
    /// the request to a blocked address (CWE-918).
    #[tokio::test]
    async fn pinned_client_does_not_follow_redirects() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();

        // Reply to any request with a redirect to a private address. If the
        // client chased it, the request would land on 127.0.0.1 (blocked by the
        // validator) instead of surfacing the raw 302.
        tokio::spawn(async move {
            if let Ok((mut sock, _)) = listener.accept().await {
                let mut buf = [0u8; 1024];
                let _ = sock.read(&mut buf).await;
                let response = "HTTP/1.1 302 Found\r\n\
                     Location: http://127.0.0.1:1/internal\r\n\
                     Content-Length: 0\r\n\r\n";
                let _ = sock.write_all(response.as_bytes()).await;
            }
        });

        let addr: SocketAddr = format!("127.0.0.1:{port}").parse().unwrap();
        let client = pinned_client("redirect.test", addr).unwrap();
        let resp = client
            .get(format!("http://redirect.test:{port}/start"))
            .send()
            .await
            .unwrap();

        assert_eq!(resp.status().as_u16(), 302);
    }
}

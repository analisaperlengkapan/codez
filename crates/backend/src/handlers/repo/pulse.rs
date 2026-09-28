use super::*;

pub async fn get_repo_pulse(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Json<RepoPulseStats> {
    let period = params.get("period").map(|s| s.as_str()).unwrap_or("weekly");
    let duration = match period {
        "daily" => Duration::days(1),
        "monthly" => Duration::days(30),
        _ => Duration::weeks(1),
    };

    let now = Utc::now();
    let start_date = now - duration;

    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return Json(RepoPulseStats {
            period: period.to_string(),
            active_issues: 0,
            closed_issues: 0,
            opened_prs: 0,
            merged_prs: 0,
            new_commits: 0,
            active_authors: vec![],
        });
    }

    let mut active_issues = 0;
    let mut closed_issues = 0;
    let mut opened_prs = 0;
    let mut merged_prs = 0;
    let mut new_commits = 0;
    let mut authors = std::collections::HashSet::new();

    let activities = state.activities.read().unwrap_or_else(|e| e.into_inner());
    for act in activities.iter() {
        if act.repo_id == repo_id {
            // Parse date
            let date = if act.created == "now" {
                Some(now)
            } else {
                chrono::DateTime::parse_from_rfc3339(&act.created)
                    .ok()
                    .map(|d| d.with_timezone(&Utc))
            };

            if let Some(d) = date {
                if d >= start_date {
                    match act.op_type.as_str() {
                        "create_issue" => {
                            active_issues += 1;
                            authors.insert((act.user_id, act.user_name.clone()));
                        }
                        "reopen_issue" => {
                            active_issues += 1;
                            authors.insert((act.user_id, act.user_name.clone()));
                        }
                        "close_issue" => {
                            closed_issues += 1;
                            authors.insert((act.user_id, act.user_name.clone()));
                        }
                        "create_pull_request" => {
                            opened_prs += 1;
                            authors.insert((act.user_id, act.user_name.clone()));
                        }
                        "merge_pull_request" => {
                            merged_prs += 1;
                            authors.insert((act.user_id, act.user_name.clone()));
                        }
                        "update_file" | "push" => {
                            // approximate commits
                            new_commits += 1;
                            authors.insert((act.user_id, act.user_name.clone()));
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    let active_authors = authors
        .into_iter()
        .map(|(id, name)| User::new(id, name, None))
        .collect();

    Json(RepoPulseStats {
        period: period.to_string(),
        active_issues,
        closed_issues,
        opened_prs,
        merged_prs,
        new_commits,
        active_authors,
    })
}

pub async fn run_security_scan(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> (StatusCode, Json<SecurityScanReport>) {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name);
    let repo_id = repo.map(|r| r.id).unwrap_or(0);

    if repo_id == 0 {
        return (
            StatusCode::NOT_FOUND,
            Json(SecurityScanReport {
                id: "".to_string(),
                repo_owner: owner,
                repo_name,
                score: 0,
                vulnerabilities: vec![],
                scanned_at: "".to_string(),
            }),
        );
    }

    let default_branch = repo
        .and_then(|r| r.default_branch.clone())
        .unwrap_or("main".to_string());
    let files = state
        .file_contents
        .read()
        .unwrap_or_else(|e| e.into_inner());

    let mut vulnerabilities = Vec::new();
    let mut vul_id = 1;

    for ((r_id, branch, path), content) in files.iter() {
        if *r_id == repo_id && branch == &default_branch {
            let lower_content = content.to_lowercase();

            // Rule 1: Secret leak detection (AWS keys, Private keys, Tokens)
            if lower_content.contains("aws_secret_access_key")
                || lower_content.contains("BEGIN PRIVATE KEY")
                || lower_content.contains("ghp_")
            {
                vulnerabilities.push(SecurityVulnerability {
                    id: format!("SEC-{}", vul_id),
                    severity: "CRITICAL".to_string(),
                    title: "Hardcoded Secret / API Token Detected".to_string(),
                    description: format!(
                        "File '{}' contains potentially exposed secrets or private credentials.",
                        path
                    ),
                    file_path: path.clone(),
                    line_no: 1,
                    recommendation:
                        "Remove secret and use Environment Variables or Repository Secrets."
                            .to_string(),
                });
                vul_id += 1;
            }

            // Rule 2: SQL Injection risks
            if lower_content.contains("select ")
                && lower_content.contains(" + ")
                && lower_content.contains("from ")
            {
                vulnerabilities.push(SecurityVulnerability {
                    id: format!("SEC-{}", vul_id),
                    severity: "HIGH".to_string(),
                    title: "Potential SQL Injection Vulnerability".to_string(),
                    description: format!(
                        "String concatenation in SQL query detected in file '{}'.",
                        path
                    ),
                    file_path: path.clone(),
                    line_no: 1,
                    recommendation: "Use parameterized queries or prepared statements.".to_string(),
                });
                vul_id += 1;
            }

            // Rule 3: Unsafe code blocks in Rust
            if path.ends_with(".rs") && lower_content.contains("unsafe {") {
                vulnerabilities.push(SecurityVulnerability {
                    id: format!("SEC-{}", vul_id),
                    severity: "MEDIUM".to_string(),
                    title: "Unsafe Code Block Detected".to_string(),
                    description: format!(
                        "Unsafe Rust block used in '{}'. Ensure boundary checks are enforced.",
                        path
                    ),
                    file_path: path.clone(),
                    line_no: 1,
                    recommendation: "Review unsafe block for potential memory safety violations."
                        .to_string(),
                });
                vul_id += 1;
            }
        }
    }

    let score = if vulnerabilities.is_empty() {
        100
    } else {
        100u8
            .saturating_sub((vulnerabilities.len() * 20) as u8)
            .max(10)
    };

    let report = SecurityScanReport {
        id: uuid::Uuid::new_v4().to_string(),
        repo_owner: owner,
        repo_name,
        score,
        vulnerabilities,
        scanned_at: Utc::now().to_rfc3339(),
    };

    (StatusCode::OK, Json(report))
}

// Helper function to extract @mentions

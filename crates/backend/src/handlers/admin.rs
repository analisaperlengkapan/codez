use axum::{
    extract::{Json, Path},
    http::StatusCode,
};
use shared::{
    Organization, Repository, Team, OrgMember, AdminStats, ActionWorkflow, Package, SystemNotice,
    TwoFactor, OAuth2Provider, DiffFile, DiffLine, Contribution, LicenseTemplate, GitignoreTemplate,
    AdminUserEditOption, User, LanguageStat, ProtectedBranch
};

pub async fn get_org(Path(org): Path<String>) -> Json<Option<Organization>> {
    if org == "codeza-org" {
        Json(Some(Organization {
            id: 1,
            username: "codeza-org".to_string(),
            description: Some("Codeza Organization".to_string()),
            avatar_url: None,
        }))
    } else {
        Json(None)
    }
}

pub async fn list_org_repos(Path(_org): Path<String>) -> Json<Vec<Repository>> {
    let repos = vec![
        Repository::new(1, "org-repo".to_string(), "codeza-org".to_string())
    ];
    Json(repos)
}

pub async fn list_teams(Path(_org): Path<String>) -> Json<Vec<Team>> {
    let teams = vec![
        Team {
            id: 1,
            name: "Developers".to_string(),
            description: Some("Dev Team".to_string()),
            permission: "write".to_string(),
        }
    ];
    Json(teams)
}

pub async fn list_org_members(Path(_org): Path<String>) -> Json<Vec<OrgMember>> {
    vec![
        OrgMember { user: User::new(1, "admin".to_string(), None), role: "owner".to_string() }
    ].into()
}

pub async fn add_org_member(Path((_org, _username)): Path<(String, String)>) -> StatusCode {
    StatusCode::CREATED
}

pub async fn remove_org_member(Path((_org, _username)): Path<(String, String)>) -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn get_admin_stats() -> Json<AdminStats> {
    Json(AdminStats {
        users: 10,
        repos: 20,
        orgs: 5,
        issues: 100,
    })
}

pub async fn list_notices() -> Json<Vec<SystemNotice>> {
    let notices = vec![
        SystemNotice { id: 1, type_: "info".to_string(), description: "System maintenance at 00:00".to_string() }
    ];
    Json(notices)
}

pub async fn admin_list_users() -> Json<Vec<User>> {
    vec![
        User::new(1, "admin".to_string(), Some("admin@example.com".to_string())),
        User::new(2, "user".to_string(), Some("user@example.com".to_string())),
    ].into()
}

pub async fn admin_edit_user(
    Path(_username): Path<String>,
    Json(_payload): Json<AdminUserEditOption>
) -> StatusCode {
    StatusCode::OK
}

pub async fn admin_delete_user(Path(_username): Path<String>) -> StatusCode {
    StatusCode::NO_CONTENT
}

// Miscellaneous Handlers
pub async fn list_workflows(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<ActionWorkflow>> {
    let wfs = vec![
        ActionWorkflow { id: 1, name: "CI".to_string(), status: "success".to_string() }
    ];
    Json(wfs)
}

pub async fn list_packages(Path(_owner): Path<String>) -> Json<Vec<Package>> {
    let pkgs = vec![
        Package { id: 1, name: "my-lib".to_string(), version: "1.0.0".to_string(), package_type: "cargo".to_string() }
    ];
    Json(pkgs)
}

pub async fn get_package_detail(Path((_owner, _type, _name, _version)): Path<(String, String, String, String)>) -> Json<Package> {
    Json(Package { id: 1, name: "pkg".to_string(), version: "1.0".to_string(), package_type: "npm".to_string() })
}

pub async fn get_2fa() -> Json<TwoFactor> {
    Json(TwoFactor { enabled: false, method: "totp".to_string() })
}

pub async fn update_2fa(Json(_payload): Json<TwoFactor>) -> StatusCode {
    StatusCode::OK
}

pub async fn list_oauth2_providers() -> Json<Vec<OAuth2Provider>> {
    let providers = vec![
        OAuth2Provider {
            name: "github".to_string(),
            display_name: "GitHub".to_string(),
            url: "http://github.com/login".to_string(),
        }
    ];
    Json(providers)
}

pub async fn get_commit_diff(Path((_owner, _repo, _sha)): Path<(String, String, String)>) -> Json<Vec<DiffFile>> {
    let diffs = vec![
        DiffFile {
            name: "src/main.rs".to_string(),
            old_name: None,
            index: "123".to_string(),
            additions: 10,
            deletions: 5,
            type_: "modify".to_string(),
            lines: vec![
                DiffLine { line_no_old: Some(1), line_no_new: Some(1), content: " fn main() {".to_string(), type_: "context".to_string() },
                DiffLine { line_no_old: Some(2), line_no_new: None, content: "-    println!(\"old\");".to_string(), type_: "delete".to_string() },
                DiffLine { line_no_old: None, line_no_new: Some(2), content: "+    println!(\"new\");".to_string(), type_: "add".to_string() },
            ],
        }
    ];
    Json(diffs)
}

pub async fn get_user_heatmap(Path(_username): Path<String>) -> Json<Vec<Contribution>> {
    vec![
        Contribution { date: "2023-01-01".to_string(), count: 5 },
        Contribution { date: "2023-01-02".to_string(), count: 2 },
    ].into()
}

pub async fn list_licenses() -> Json<Vec<LicenseTemplate>> {
    vec![
        LicenseTemplate { key: "mit".to_string(), name: "MIT License".to_string(), url: "http://...".to_string() }
    ].into()
}

pub async fn list_gitignores() -> Json<Vec<GitignoreTemplate>> {
    vec![
        GitignoreTemplate { name: "Rust".to_string(), source: "target/".to_string() }
    ].into()
}

pub async fn get_repo_languages(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<LanguageStat>> {
    vec![
        LanguageStat { language: "Rust".to_string(), percentage: 100, color: "#dea584".to_string() }
    ].into()
}

pub async fn list_branch_protections(Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<ProtectedBranch>> {
    vec![
        ProtectedBranch { name: "main".to_string(), enable_push: false, enable_force_push: false }
    ].into()
}

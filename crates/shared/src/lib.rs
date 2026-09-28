//! Shared DTOs and helpers used by both the backend API and the Leptos frontend.
//!
//! The crate is organised into one module per domain; every type is re-exported at
//! the crate root so downstream code keeps using `shared::<Type>` paths.

pub mod actions;
pub mod contents;
pub mod discussions;
pub mod issues;
pub mod migration;
pub mod organizations;
pub mod projects;
pub mod pulls;
pub mod releases;
pub mod repository;
pub mod security;
pub mod users;
pub mod webhooks;
pub mod wiki;

pub use actions::*;
pub use contents::*;
pub use discussions::*;
pub use issues::*;
pub use migration::*;
pub use organizations::*;
pub use projects::*;
pub use pulls::*;
pub use releases::*;
pub use repository::*;
pub use security::*;
pub use users::*;
pub use webhooks::*;
pub use wiki::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(
            1,
            "jules".to_string(),
            Some("jules@example.com".to_string()),
        );
        assert_eq!(user.username, "jules");
        assert_eq!(user.email, Some("jules@example.com".to_string()));
    }

    #[test]
    fn test_collab_branch_tag() {
        let user = User::new(1, "u".to_string(), None);
        let commit = Commit {
            sha: "s".to_string(),
            repo_id: 1,
            message: "m".to_string(),
            author: user.clone(),
            date: "d".to_string(),
        };

        let c = Collaborator {
            user: user.clone(),
            repo_id: 1,
            permissions: "read".to_string(),
        };
        assert_eq!(c.permissions, "read");

        let b = Branch {
            name: "main".to_string(),
            repo_id: 1,
            commit: commit.clone(),
            protected: true,
        };
        assert!(b.protected);

        let t = Tag {
            name: "v1".to_string(),
            repo_id: 1,
            id: "1".to_string(),
            commit: commit.clone(),
        };
        assert_eq!(t.name, "v1");
    }

    #[test]
    fn test_system_notices_2fa() {
        let notice = SystemNotice {
            id: 1,
            type_: "alert".to_string(),
            description: "System update".to_string(),
        };
        assert_eq!(notice.type_, "alert");

        let two_fa = TwoFactor {
            enabled: true,
            method: "totp".to_string(),
        };
        assert!(two_fa.enabled);
    }

    #[test]
    fn test_lfs_oauth_reaction() {
        let lfs = LfsObject {
            oid: "oid".to_string(),
            size: 100,
            created_at: "now".to_string(),
        };
        assert_eq!(lfs.size, 100);

        let oauth = OAuth2Provider {
            name: "github".to_string(),
            display_name: "GitHub".to_string(),
            url: "http".to_string(),
        };
        assert_eq!(oauth.name, "github");

        let user = User::new(1, "u".to_string(), None);
        let react = Reaction {
            id: 1,
            user,
            content: "heart".to_string(),
            created_at: "now".to_string(),
        };
        assert_eq!(react.content, "heart");

        let opt = CreateReactionOption {
            content: "+1".to_string(),
        };
        assert_eq!(opt.content, "+1");
    }

    #[test]
    fn test_diff_structs() {
        let line = DiffLine {
            line_no_old: Some(1),
            line_no_new: Some(1),
            content: "code".to_string(),
            type_: "context".to_string(),
        };
        assert_eq!(line.content, "code");

        let file = DiffFile {
            name: "file.rs".to_string(),
            old_name: None,
            index: "idx".to_string(),
            additions: 1,
            deletions: 0,
            type_: "modify".to_string(),
            lines: vec![line],
        };
        assert_eq!(file.name, "file.rs");
    }

    #[test]
    fn test_contribution_org_member() {
        let c = Contribution {
            date: "2023-01-01".to_string(),
            count: 5,
        };
        assert_eq!(c.count, 5);

        let user = User::new(1, "u".to_string(), None);
        let m = OrgMember {
            user,
            role: "owner".to_string(),
        };
        assert_eq!(m.role, "owner");
    }

    #[test]
    fn test_secret_deploykey() {
        let s = Secret {
            name: "TOKEN".to_string(),
            repo_id: 1,
            created_at: "now".to_string(),
            data: "d".to_string(),
        };
        assert_eq!(s.name, "TOKEN");

        let k = DeployKey {
            id: 1,
            repo_id: 1,
            title: "deploy".to_string(),
            key: "k".to_string(),
            fingerprint: "f".to_string(),
        };
        assert_eq!(k.title, "deploy");
    }

    #[test]
    fn test_actions_packages_structs() {
        let wf = ActionWorkflow {
            id: 1,
            repo_id: 1,
            name: "build".to_string(),
            status: "success".to_string(),
        };
        assert_eq!(wf.name, "build");
        assert_eq!(wf.repo_id, 1);

        let pkg = Package {
            id: 1,
            owner: "admin".to_string(),
            name: "pkg".to_string(),
            version: "1.0".to_string(),
            package_type: "npm".to_string(),
        };
        assert_eq!(pkg.package_type, "npm");
    }

    #[test]
    fn test_activity_struct() {
        let act = Activity {
            id: 1,
            repo_id: 1,
            user_id: 1,
            user_name: "u".to_string(),
            op_type: "push".to_string(),
            content: "c".to_string(),
            created: "d".to_string(),
        };
        assert_eq!(act.op_type, "push");
    }

    #[test]
    fn test_admin_stats() {
        let stats = AdminStats {
            users: 10,
            repos: 20,
            orgs: 5,
            issues: 100,
        };
        assert_eq!(stats.users, 10);
    }

    #[test]
    fn test_team_project_structs() {
        let team = Team {
            id: 1,
            org_name: "org".to_string(),
            name: "dev".to_string(),
            description: None,
            permission: "write".to_string(),
        };
        assert_eq!(team.name, "dev");

        let project = Project {
            id: 1,
            repo_id: 1,
            title: "v1".to_string(),
            description: None,
            is_closed: false,
        };
        assert!(!project.is_closed);
    }

    #[test]
    fn test_keys_hooks_structs() {
        let key = PublicKey {
            id: 1,
            title: "Laptop".to_string(),
            key: "ssh-rsa...".to_string(),
            fingerprint: "sha256...".to_string(),
        };
        assert_eq!(key.title, "Laptop");

        let hook = Webhook {
            id: 1,
            repo_id: 1,
            url: "http://example.com".to_string(),
            events: vec!["push".to_string()],
            active: true,
        };
        assert!(hook.active);
    }

    #[test]
    fn test_wiki_structs() {
        let page = WikiPage {
            title: "Home".to_string(),
            content: "Welcome".to_string(),
            commit_message: None,
        };
        assert_eq!(page.title, "Home");
    }

    #[test]
    fn test_settings_structs() {
        let r_opts = RepoSettingsOption {
            description: Some("desc".to_string()),
            private: Some(true),
            website: None,
            default_branch: Some("main".to_string()),
            allow_rebase_merge: None,
            allow_squash_merge: None,
            allow_merge_commit: None,
            has_issues: None,
            has_wiki: None,
            has_projects: None,
        };
        assert_eq!(r_opts.description, Some("desc".to_string()));

        let u_opts = UserSettingsOption {
            full_name: Some("Name".to_string()),
            website: None,
            description: None,
            location: None,
        };
        assert_eq!(u_opts.full_name, Some("Name".to_string()));
    }

    #[test]
    fn test_topic_structs() {
        let topic = Topic {
            id: 1,
            repo_id: 1,
            name: "rust".to_string(),
            created: "date".to_string(),
        };
        assert_eq!(topic.name, "rust");

        let opts = RepoTopicOptions {
            topics: vec!["rust".to_string(), "gitea".to_string()],
        };
        assert_eq!(opts.topics.len(), 2);
    }

    #[test]
    fn test_search_struct() {
        let search = RepoSearchOptions {
            q: "test".to_string(),
            uid: Some(1),
        };
        assert_eq!(search.q, "test");
    }

    #[test]
    fn test_label_structs() {
        let label = Label {
            id: 1,
            repo_id: 1,
            name: "bug".to_string(),
            color: "#ff0000".to_string(),
            description: None,
        };
        assert_eq!(label.name, "bug");

        let opts = CreateLabelOption {
            name: "feature".to_string(),
            color: "#00ff00".to_string(),
            description: None,
        };
        assert_eq!(opts.color, "#00ff00");
    }

    #[test]
    fn test_milestone_structs() {
        let milestone = Milestone {
            id: 1,
            repo_id: 1,
            title: "v1.0".to_string(),
            description: None,
            due_on: None,
            state: "open".to_string(),
        };
        assert_eq!(milestone.title, "v1.0");

        let opts = CreateMilestoneOption {
            title: "v2.0".to_string(),
            description: None,
            due_on: None,
        };
        assert_eq!(opts.title, "v2.0");
    }

    #[test]
    fn test_comment_structs() {
        let user = User::new(1, "u".to_string(), None);
        let comment = Comment {
            id: 1,
            issue_id: 1,
            body: "text".to_string(),
            user,
            created_at: "date".to_string(),
            reactions: vec![],
        };
        assert_eq!(comment.body, "text");

        let merge = MergePullRequestOption {
            merge_action: "merge".to_string(),
            merge_message_field: None,
            merge_title_field: None,
        };
        assert_eq!(merge.merge_action, "merge");
    }

    #[test]
    fn test_auth_structs() {
        let login = LoginOption {
            username: "u".to_string(),
            password: "p".to_string(),
        };
        assert_eq!(login.username, "u");

        let reg = RegisterOption {
            username: "u".to_string(),
            email: "e".to_string(),
            password: "p".to_string(),
        };
        assert_eq!(reg.email, "e");
    }

    #[test]
    fn test_org_struct() {
        let org = Organization {
            id: 1,
            username: "org".to_string(),
            description: None,
            avatar_url: None,
            website: None,
            location: None,
            email: None,
            visibility: None,
        };
        assert_eq!(org.username, "org");
    }

    #[test]
    fn test_release_structs() {
        let user = User::new(1, "u".to_string(), None);
        let rel = Release {
            id: 1,
            repo_id: 1,
            tag_name: "v1.0".to_string(),
            name: "Release 1.0".to_string(),
            body: None,
            draft: false,
            prerelease: false,
            created_at: "date".to_string(),
            author: user,
            assets: vec![],
        };
        assert_eq!(rel.tag_name, "v1.0");

        let opts = CreateReleaseOption {
            tag_name: "v1.1".to_string(),
            name: "Next".to_string(),
            body: None,
            draft: true,
            prerelease: false,
        };
        assert!(opts.draft);
    }

    #[test]
    fn test_commit() {
        let user = User::new(1, "committer".to_string(), None);
        let commit = Commit {
            sha: "abc1234".to_string(),
            repo_id: 1,
            message: "Initial commit".to_string(),
            author: user,
            date: "2023-01-01".to_string(),
        };
        assert_eq!(commit.sha, "abc1234");
    }

    #[test]
    fn test_file_entry() {
        let file = FileEntry {
            name: "README.md".to_string(),
            path: "README.md".to_string(),
            kind: "file".to_string(),
            size: 1024,
        };
        assert_eq!(file.name, "README.md");
        assert_eq!(file.kind, "file");
    }

    #[test]
    fn test_pull_request_structs() {
        let user = User::new(1, "user".to_string(), None);
        let pr = PullRequest {
            id: 1,
            repo_id: 1,
            number: 1,
            title: "PR Title".to_string(),
            body: None,
            state: "open".to_string(),
            user,
            merged: false,
            head_sha: "sha".to_string(),
            base: "main".to_string(),
            head: "feature".to_string(),
        };
        assert_eq!(pr.title, "PR Title");
        assert!(!pr.merged);

        let opts = CreatePullRequestOption {
            title: "New Feature".to_string(),
            body: None,
            head: "feature".to_string(),
            base: "main".to_string(),
        };
        assert_eq!(opts.head, "feature");
    }

    #[test]
    fn test_issue_structs() {
        let user = User::new(1, "user".to_string(), None);
        let issue = Issue {
            id: 1,
            repo_id: 1,
            number: 1,
            title: "Bug".to_string(),
            body: None,
            state: "open".to_string(),
            user,
            assignees: vec![],
            labels: vec![],
            milestone: None,
            is_locked: false,
        };
        assert_eq!(issue.title, "Bug");

        let opts = CreateIssueOption {
            title: "New Bug".to_string(),
            body: Some("Description".to_string()),
            milestone: None,
        };
        assert_eq!(opts.title, "New Bug");

        let update = UpdateIssueOption {
            title: Some("Updated".to_string()),
            body: None,
            state: Some("closed".to_string()),
            milestone_id: None,
        };
        assert_eq!(update.title, Some("Updated".to_string()));

        let filter = IssueFilterOptions {
            state: Some("open".to_string()),
            q: Some("bug".to_string()),
            label_id: Some(1),
            assignee_username: Some("admin".to_string()),
            milestone_id: Some(2),
            page: Some(1),
            limit: Some(10),
            sort: Some("created".to_string()),
            direction: Some("desc".to_string()),
        };
        assert_eq!(filter.state.unwrap(), "open");
        assert_eq!(filter.milestone_id.unwrap(), 2);
    }

    #[test]
    fn test_create_repo_option() {
        let opts = CreateRepoOption {
            name: "new-repo".to_string(),
            description: Some("desc".to_string()),
            private: true,
            auto_init: false,
            gitignores: Some("Rust".to_string()),
            license: Some("MIT".to_string()),
            readme: Some("Default".to_string()),
            default_branch: None,
            allow_rebase_merge: None,
            allow_squash_merge: None,
            allow_merge_commit: None,
            has_issues: None,
            has_wiki: None,
            has_projects: None,
        };
        assert_eq!(opts.name, "new-repo");
        assert!(opts.private);
        assert_eq!(opts.license, Some("MIT".to_string()));
    }

    #[test]
    fn test_repository_creation() {
        let repo = Repository::new(1, "codeza".to_string(), "jules".to_string());
        assert_eq!(repo.id, 1);
        assert_eq!(repo.name, "codeza");
        assert_eq!(repo.owner, "jules");
        assert!(!repo.private);
        assert_eq!(repo.stars_count, 0);
        assert!(!repo.is_mirror);
    }

    #[test]
    fn test_gpg_key() {
        let key = GpgKey {
            id: 1,
            key_id: "ID".to_string(),
            primary_key_id: "PID".to_string(),
            public_key: "PUB".to_string(),
            emails: vec![],
        };
        assert_eq!(key.key_id, "ID");
    }

    #[test]
    fn test_repo_action() {
        let act = RepoActionOption {
            action: "star".to_string(),
        };
        assert_eq!(act.action, "star");
    }

    #[test]
    fn test_audit_log_struct() {
        let user = User::new(1, "admin".to_string(), None);
        let log = AuditLog {
            id: 1,
            actor: user,
            action: "org.member_role_update".to_string(),
            target_type: "org".to_string(),
            target_name: "codeza-org".to_string(),
            details: "Changed role of user 'jules' to maintainer".to_string(),
            ip_address: Some("127.0.0.1".to_string()),
            created_at: "now".to_string(),
        };
        assert_eq!(log.action, "org.member_role_update");
        assert_eq!(log.target_name, "codeza-org");
    }

    #[test]
    fn test_repository_serialization() {
        let repo = Repository::new(1, "codeza".to_string(), "jules".to_string());
        let json = serde_json::to_string(&repo).unwrap();
        let deserialized: Repository = serde_json::from_str(&json).unwrap();
        assert_eq!(repo, deserialized);
    }

    #[test]
    fn test_templates() {
        let l = LicenseTemplate {
            key: "mit".to_string(),
            name: "MIT".to_string(),
            url: "u".to_string(),
        };
        assert_eq!(l.key, "mit");
        let g = GitignoreTemplate {
            name: "Rust".to_string(),
            source: "target/".to_string(),
        };
        assert_eq!(g.name, "Rust");
    }

    #[test]
    fn test_review_admin_structs() {
        let u = User::new(1, "u".to_string(), None);
        let rr = ReviewRequest {
            reviewer: u,
            status: "s".to_string(),
        };
        assert_eq!(rr.status, "s");

        let a = AdminUserEditOption {
            email: Some("e".to_string()),
            password: None,
            active: Some(true),
            admin: None,
        };
        assert!(a.active.unwrap());
    }

    #[test]
    fn test_lang_prot_email_app() {
        let l = LanguageStat {
            language: "Rust".to_string(),
            percentage: 100,
            color: "#dea584".to_string(),
        };
        assert_eq!(l.percentage, 100);

        let pb = ProtectedBranch {
            id: 1,
            repo_id: 1,
            name: "main".to_string(),
            enable_push: false,
            enable_force_push: false,
            required_status_checks: vec![],
        };
        assert!(!pb.enable_push);

        let e = EmailAddress {
            email: "e".to_string(),
            verified: true,
            primary: true,
        };
        assert!(e.primary);

        let app = OAuth2Application {
            id: 1,
            name: "app".to_string(),
            client_id: "cid".to_string(),
            client_secret: "secret".to_string(),
            redirect_uris: vec![],
        };
        assert_eq!(app.client_id, "cid");
    }

    #[test]
    fn test_migrate_transfer_options() {
        let m = MigrateRepoOption {
            clone_addr: "url".to_string(),
            repo_name: "name".to_string(),
            service: "git".to_string(),
            mirror: true,
        };
        assert!(m.mirror);

        let t = TransferRepoOption {
            new_owner: "new".to_string(),
        };
        assert_eq!(t.new_owner, "new");
    }

    #[test]
    fn test_milestone_stats() {
        let stats = MilestoneStats {
            open_issues: 10,
            closed_issues: 5,
        };
        assert_eq!(stats.open_issues, 10);
    }

    #[test]
    fn test_code_search() {
        let r = CodeSearchResult {
            name: "n".to_string(),
            path: "p".to_string(),
            sha: "s".to_string(),
            url: "u".to_string(),
            content: Some("c".to_string()),
        };
        assert_eq!(r.name, "n");
    }

    #[test]
    fn test_lfs_lock() {
        let u = User::new(1, "u".to_string(), None);
        let l = LfsLock {
            id: "1".to_string(),
            repo_id: 1,
            path: "p".to_string(),
            owner: u,
            locked_at: "t".to_string(),
        };
        assert_eq!(l.path, "p");
    }
}

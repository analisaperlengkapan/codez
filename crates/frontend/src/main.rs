use leptos::*;
use leptos_router::*;
use shared::{CreateRepoOption, FileEntry, Issue, PullRequest, Repository, User};

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <a href="/">"Home"</a> " | "
                <a href="/users/admin">"Admin Profile"</a> " | "
                <a href="/repo/create">"New Repo"</a>
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/repo/create" view=CreateRepo/>
                    <Route path="/users/:username" view=UserProfile/>
                    <Route path="/repos/:owner/:repo" view=RepoDetail/>
                    <Route path="/repos/:owner/:repo/issues" view=IssueList/>
                    <Route path="/repos/:owner/:repo/pulls" view=PullRequestList/>
                    <Route path="/repos/:owner/:repo/src/*path" view=RepoCode/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (repos, set_repos) = create_signal(vec![]);

    create_effect(move |_| {
        // Mock fetch
        let mock_repos = vec![
            Repository::new(1, "codeza".to_string(), "admin".to_string()),
            Repository::new(2, "gitea-clone".to_string(), "user".to_string()),
        ];
        set_repos.set(mock_repos);
    });

    view! {
        <div class="container">
            <h1>"Repositories"</h1>
            <ul>
                <For
                    each=move || repos.get()
                    key=|repo| repo.id
                    children=move |repo| {
                        let href = format!("/repos/{}/{}", repo.owner, repo.name);
                        view! {
                            <li>
                                <a href=href>{repo.owner} " / " {repo.name}</a>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn UserProfile() -> impl IntoView {
    let params = use_params_map();
    let username = move || params.with(|params| params.get("username").cloned().unwrap_or_default());

    // Mock user fetch
    let user = create_memo(move |_| {
         if username() == "admin" {
             Some(User::new(1, "admin".to_string(), Some("admin@codeza.com".to_string())))
         } else {
             None
         }
    });

    view! {
        <div class="user-profile">
            <h2>"User Profile: " {username}</h2>
            {move || match user.get() {
                Some(u) => view! {
                    <p>"Email: " {u.email.unwrap_or("Hidden".to_string())}</p>
                }.into_view(),
                None => view! { <p>"User not found"</p> }.into_view()
            }}
        </div>
    }
}

#[component]
fn CreateRepo() -> impl IntoView {
    let (name, set_name) = create_signal("".to_string());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = CreateRepoOption {
            name: name.get(),
            description: None,
            private: false,
            auto_init: true,
        };
        // In a real app, we would POST this payload
        logging::log!("Creating repo: {:?}", payload);
    };

    view! {
        <div class="create-repo">
            <h3>"Create New Repository"</h3>
            <form on:submit=on_submit>
                <input
                    type="text"
                    placeholder="Repository Name"
                    prop:value=name
                    on:input=move |ev| set_name.set(event_target_value(&ev))
                />
                <button type="submit">"Create"</button>
            </form>
        </div>
    }
}

#[component]
fn RepoDetail() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let issues_href = move || format!("/repos/{}/{}/issues", owner(), repo_name());
    let pulls_href = move || format!("/repos/{}/{}/pulls", owner(), repo_name());
    let code_href = move || format!("/repos/{}/{}/src/", owner(), repo_name());

    view! {
        <div class="repo-detail">
            <h3>"Repository: " {owner} " / " {repo_name}</h3>
            <p>"Clone URL: https://codeza.com/" {owner} "/" {repo_name} ".git"</p>
            <p>
                <a href=code_href>"Code"</a> " | "
                <a href=issues_href>"Issues"</a> " | "
                <a href=pulls_href>"Pull Requests"</a>
            </p>
        </div>
    }
}

#[component]
fn RepoCode() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());
    let path = move || params.with(|params| params.get("path").cloned().unwrap_or_default());

    let (files, set_files) = create_signal(vec![]);

    create_effect(move |_| {
        // Mock contents
        let mut mock = vec![];
        if path() == "" || path() == "/" {
            mock.push(FileEntry { name: "src".to_string(), path: "src".to_string(), kind: "dir".to_string(), size: 0 });
            mock.push(FileEntry { name: "README.md".to_string(), path: "README.md".to_string(), kind: "file".to_string(), size: 1024 });
        } else if path() == "src" {
             mock.push(FileEntry { name: "main.rs".to_string(), path: "src/main.rs".to_string(), kind: "file".to_string(), size: 512 });
        }
        set_files.set(mock);
    });

    view! {
        <div class="repo-code">
            <h3>"Code: " {owner} " / " {repo_name} " / " {path}</h3>
            <ul>
                <For
                    each=move || files.get()
                    key=|f| f.path.clone()
                    children=move |f| {
                        let href = format!("/repos/{}/{}/src/{}", owner(), repo_name(), f.path);
                        view! {
                            <li>
                                <a href=href>{f.name}</a> " (" {f.kind} ")"
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn PullRequestList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (pulls, set_pulls) = create_signal(vec![]);

    create_effect(move |_| {
        // Mock fetch
        let user = User::new(1, "admin".to_string(), None);
        let mock = vec![
             PullRequest {
                id: 1,
                number: 1,
                title: "First PR".to_string(),
                body: Some("Desc".to_string()),
                state: "open".to_string(),
                user,
                merged: false,
            }
        ];
        set_pulls.set(mock);
    });

    view! {
        <div class="pull-list">
             <h3>"Pull Requests for " {owner} " / " {repo_name}</h3>
             <ul>
                <For
                    each=move || pulls.get()
                    key=|pr| pr.id
                    children=move |pr| {
                        view! {
                            <li>
                                <strong>"#" {pr.number} " " {pr.title}</strong>
                                " (" {pr.state} ") by " {pr.user.username}
                            </li>
                        }
                    }
                />
             </ul>
        </div>
    }
}

#[component]
fn IssueList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (issues, set_issues) = create_signal(vec![]);

    create_effect(move |_| {
        // Mock fetch
        let user = User::new(1, "admin".to_string(), None);
        let mock = vec![
             Issue {
                id: 1,
                number: 1,
                title: "First Issue".to_string(),
                body: Some("Bug report".to_string()),
                state: "open".to_string(),
                user,
            }
        ];
        set_issues.set(mock);
    });

    view! {
        <div class="issue-list">
             <h3>"Issues for " {owner} " / " {repo_name}</h3>
             <ul>
                <For
                    each=move || issues.get()
                    key=|issue| issue.id
                    children=move |issue| {
                        view! {
                            <li>
                                <strong>"#" {issue.number} " " {issue.title}</strong>
                                " (" {issue.state} ") by " {issue.user.username}
                            </li>
                        }
                    }
                />
             </ul>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_home_logic() {
        // Simple verification that types align
        let repo = Repository::new(1, "test".to_string(), "me".to_string());
        assert_eq!(repo.name, "test");
    }

    #[test]
    fn test_create_repo_logic() {
        let opts = CreateRepoOption {
            name: "test".to_string(),
            description: None,
            private: false,
            auto_init: true,
        };
        assert_eq!(opts.name, "test");
    }

    #[test]
    fn test_issue_logic() {
        let user = User::new(1, "u".to_string(), None);
        let issue = Issue {
            id: 1,
            number: 1,
            title: "t".to_string(),
            body: None,
            state: "o".to_string(),
            user
        };
        assert_eq!(issue.title, "t");
    }

    #[test]
    fn test_pr_logic() {
        let user = User::new(1, "u".to_string(), None);
        let pr = PullRequest {
            id: 1,
            number: 1,
            title: "p".to_string(),
            body: None,
            state: "o".to_string(),
            user,
            merged: false,
        };
        assert_eq!(pr.title, "p");
    }

    #[test]
    fn test_file_entry_logic() {
        let f = FileEntry {
            name: "n".to_string(),
            path: "p".to_string(),
            kind: "f".to_string(),
            size: 1,
        };
        assert_eq!(f.name, "n");
    }
}

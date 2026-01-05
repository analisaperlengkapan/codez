use leptos::*;
use leptos_router::*;
use shared::{CreateRepoOption, Repository, User};

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

    view! {
        <div class="repo-detail">
            <h3>"Repository: " {owner} " / " {repo_name}</h3>
            <p>"Clone URL: https://codeza.com/" {owner} "/" {repo_name} ".git"</p>
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
}

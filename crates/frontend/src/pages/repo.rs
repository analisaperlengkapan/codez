use leptos::*;
use leptos_router::*;
use gloo_net::http::Request;
use shared::{Repository, CreateRepoOption, Package, FileEntry, Issue};

#[component]
pub fn RepoDetail() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let repo = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}", o, r)).send().await.unwrap().json::<Option<Repository>>().await.unwrap_or(None)
        }
    );

    view! {
        <div class="repo-detail">
            <h3>"Repository: " {owner} " / " {repo_name}</h3>
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || match repo.get() {
                    Some(Some(r)) => view! {
                        <p>"Clone URL: https://codeza.com/" {r.owner} "/" {r.name} ".git"</p>
                        <p>
                            <a href="issues">"Issues"</a> " | " <a href="pulls">"Pull Requests"</a> " | " <a href="src">"Code"</a> " | " <a href="commits">"Commits"</a>
                        </p>
                    }.into_view(),
                    _ => view! { <p>"Repo not found"</p> }.into_view()
                }}
            </Suspense>
        </div>
    }
}

#[component]
pub fn RepoCode() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());
    // "path" param handles the *path wildcard in routing
    let path = move || params.with(|params| params.get("path").cloned().unwrap_or_default());

    let contents = create_resource(
        move || (owner(), repo_name(), path()),
        |(o, r, p)| async move {
            let url = if p.is_empty() {
                format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/contents", o, r)
            } else {
                format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/contents/{}", o, r, p)
            };
            Request::get(&url).send().await.unwrap().json::<Vec<FileEntry>>().await.unwrap_or_default()
        }
    );

    view! {
        <div class="repo-code">
            <h3>"Files in " {move || if path().is_empty() { "root".to_string() } else { path() }}</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading files..."</li> }>
                    {move || contents.get().map(|files| {
                        if files.is_empty() {
                             view! { <li>"No files found or empty directory."</li> }.into_view()
                        } else {
                            view! {
                                <For each=move || files.clone() key=|f| f.path.clone() children=move |f| {
                                    let is_dir = f.kind == "dir";
                                    let link = format!("/repos/{}/{}/src/{}", owner(), repo_name(), f.path);

                                    view! {
                                        <li>
                                            {if is_dir { "📁 " } else { "📄 " }}
                                            <a href=link>{f.name}</a>
                                            " (" {f.size} " bytes)"
                                        </li>
                                    }
                                }/>
                            }.into_view()
                        }
                    })}
                </Suspense>
            </ul>
        </div>
    }
}

#[component]
pub fn IssueList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let issues = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/issues", o, r))
                .send().await.unwrap().json::<Vec<Issue>>().await.unwrap_or_default()
        }
    );

    view! {
        <div class="issue-list">
            <h3>"Issues for " {owner} "/" {repo_name}</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading issues..."</li> }>
                    {move || issues.get().map(|list| view! {
                        <For each=move || list.clone() key=|i| i.id children=move |i| {
                            let href = format!("/repos/{}/{}/issues/{}", owner(), repo_name(), i.id);
                            view! { <li><a href=href>"#" {i.number} " " {i.title}</a> " (" {i.state} ")"</li> }
                        }/>
                    })}
                </Suspense>
            </ul>
        </div>
    }
}

#[component]
pub fn CreateRepo() -> impl IntoView {
    let (name, set_name) = create_signal("".to_string());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = CreateRepoOption {
            name: name.get(),
            description: None,
            private: false,
            auto_init: true,
        };
        spawn_local(async move {
            let _ = Request::post("http://127.0.0.1:3000/api/v1/user/repos").json(&payload).unwrap().send().await;
        });
    };

    view! {
        <div class="create-repo">
            <h3>"Create New Repository"</h3>
            <form on:submit=on_submit>
                <input type="text" placeholder="Repository Name" prop:value=name on:input=move |ev| set_name.set(event_target_value(&ev)) />
                <button type="submit">"Create"</button>
            </form>
        </div>
    }
}

#[component]
pub fn PackageList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());

    let packages = create_resource(owner, |owner_name| async move {
        Request::get(&format!("http://127.0.0.1:3000/api/v1/packages/{}", owner_name)).send().await.unwrap().json::<Vec<Package>>().await.unwrap_or_default()
    });

    view! {
        <div class="package-list">
            <h3>"Packages for " {owner}</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading..."</li> }>
                    {move || packages.get().map(|list| view! {
                        <For each=move || list.clone() key=|p| p.id children=move |p| {
                            let href = format!("/packages/{}/{}/{}/{}", owner(), p.package_type, p.name, p.version);
                            view! { <li><a href=href>{p.name} " (" {p.package_type} ") - " {p.version}</a></li> }
                        }/>
                    })}
                </Suspense>
            </ul>
        </div>
    }
}

#[component]
pub fn PackageDetail() -> impl IntoView {
    let params = use_params_map();
    let name = move || params.with(|params| params.get("name").cloned().unwrap_or_default());
    let version = move || params.with(|params| params.get("version").cloned().unwrap_or_default());

    view! {
        <div class="package-detail">
            <h3>"Package: " {name} " " {version}</h3>
            <p>"Installation instructions..."</p>
        </div>
    }
}

// Stub components to allow compilation
#[component]
pub fn RepoCodeSearch() -> impl IntoView { view! { <div>"Repo Search Placeholder"</div> } }
#[component]
pub fn ActionsList() -> impl IntoView { view! { <div>"Actions List Placeholder"</div> } }
#[component]
pub fn BranchList() -> impl IntoView { view! { <div>"Branch List Placeholder"</div> } }
#[component]
pub fn TagList() -> impl IntoView { view! { <div>"Tag List Placeholder"</div> } }
#[component]
pub fn CommitList() -> impl IntoView { view! { <div>"Commit List Placeholder"</div> } }
#[component]
pub fn CommitDiff() -> impl IntoView { view! { <div>"Commit Diff Placeholder"</div> } }
#[component]
pub fn ReleaseList() -> impl IntoView { view! { <div>"Release List Placeholder"</div> } }
#[component]
pub fn LabelList() -> impl IntoView { view! { <div>"Label List Placeholder"</div> } }
#[component]
pub fn MilestoneList() -> impl IntoView { view! { <div>"Milestone List Placeholder"</div> } }
#[component]
pub fn MilestoneDetail() -> impl IntoView { view! { <div>"Milestone Detail Placeholder"</div> } }
#[component]
pub fn ProjectList() -> impl IntoView { view! { <div>"Project List Placeholder"</div> } }
#[component]
pub fn Wiki() -> impl IntoView { view! { <div>"Wiki Placeholder"</div> } }
#[component]
pub fn WikiEdit() -> impl IntoView { view! { <div>"Wiki Edit Placeholder"</div> } }
#[component]
pub fn RepoSettings() -> impl IntoView { view! { <div>"Repo Settings Placeholder"</div> } }
#[component]
pub fn IssueDetail() -> impl IntoView { view! { <div>"Issue Detail Placeholder"</div> } }
#[component]
pub fn PullRequestList() -> impl IntoView { view! { <div>"Pull Request List Placeholder"</div> } }
#[component]
pub fn PullRequestDetail() -> impl IntoView { view! { <div>"Pull Request Detail Placeholder"</div> } }

use leptos::*;
use leptos_router::*;
use gloo_net::http::Request;
use shared::{Issue, Comment, PullRequest};

#[component]
pub fn IssueList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let issues = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/issues", o, r)).send().await.unwrap().json::<Vec<Issue>>().await.unwrap_or_default()
        }
    );

    view! {
        <div class="issue-list">
             <h3>"Issues for " {owner} " / " {repo_name}</h3>
             <ul>
                <Suspense fallback=move || view! { <li>"Loading..."</li> }>
                    {move || issues.get().map(|list| view! {
                        <For each=move || list.clone() key=|i| i.id children=move |i| {
                            view! { <li>"#" {i.number} " " {i.title} " (" {i.state} ")"</li> }
                        }/>
                    })}
                </Suspense>
             </ul>
        </div>
    }
}

// Stubs for now
#[component]
pub fn IssueDetail() -> impl IntoView { view! { <div>"Issue Detail Placeholder"</div> } }
#[component]
pub fn PullRequestList() -> impl IntoView { view! { <div>"PR List Placeholder"</div> } }
#[component]
pub fn PullRequestDetail() -> impl IntoView { view! { <div>"PR Detail Placeholder"</div> } }

use leptos::*;
use gloo_net::http::Request;
use shared::{Activity, Repository};
use crate::components::activity::ActivityFeed;

async fn fetch_repos() -> Vec<Repository> {
    let resp = Request::get("http://127.0.0.1:3000/api/v1/repos").send().await.unwrap();
    resp.json().await.unwrap_or_default()
}

#[component]
pub fn UserDashboard() -> impl IntoView {
    let repos_resource = create_resource(|| (), |_| async move { fetch_repos().await });

    view! {
        <div class="dashboard container">
            <div class="dashboard-sidebar">
                <h3>"My Repositories"</h3>
                <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                    {move || {
                        repos_resource.get().map(|repos| view! {
                            <ul>
                                <For each=move || repos.clone() key=|repo| repo.id children=move |repo| {
                                    view! { <li><a href=format!("/repos/{}/{}", repo.owner, repo.name)>{repo.name}</a></li> }
                                }/>
                            </ul>
                        })
                    }}
                </Suspense>
                <h3>"Organizations"</h3>
                <p><a href="/orgs/codeza-org">"codeza-org"</a></p>
            </div>
            <div class="dashboard-content">
                <ActivityFeed/>
            </div>
        </div>
    }
}

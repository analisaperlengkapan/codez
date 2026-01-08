use leptos::*;
use gloo_net::http::Request;
use leptos_router::*;
use shared::{Organization, OrgMember, Repository, Team};

#[component]
pub fn OrgProfile() -> impl IntoView {
    let params = use_params_map();
    let org_name = move || params.with(|params| params.get("org").cloned().unwrap_or_default());

    let org = create_resource(org_name, |org| async move {
        Request::get(&format!("http://127.0.0.1:3000/api/v1/orgs/{}", org)).send().await.unwrap().json::<Option<Organization>>().await.unwrap_or(None)
    });

    view! {
        <div class="org-profile">
            <h2>"Organization: " {org_name}</h2>
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || match org.get() {
                    Some(Some(o)) => view! {
                        <div>
                            <p>{o.description.unwrap_or_default()}</p>
                            <OrgRepos/>
                            <OrgTeams/>
                            <OrgMembers/>
                        </div>
                    }.into_view(),
                    _ => view! { <p>"Org not found"</p> }.into_view()
                }}
            </Suspense>
        </div>
    }
}

#[component]
pub fn OrgMembers() -> impl IntoView {
    let params = use_params_map();
    let org = move || params.with(|params| params.get("org").cloned().unwrap_or_default());

    let members = create_resource(org, |org_name| async move {
        Request::get(&format!("http://127.0.0.1:3000/api/v1/orgs/{}/members", org_name)).send().await.unwrap().json::<Vec<OrgMember>>().await.unwrap_or_default()
    });

    view! {
        <div class="org-members">
            <h3>"Members"</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading..."</li> }>
                    {move || members.get().map(|list| view! {
                        <For each=move || list.clone() key=|m| m.user.id children=move |m| {
                            view! { <li>{m.user.username} " (" {m.role} ")"</li> }
                        }/>
                    })}
                </Suspense>
            </ul>
        </div>
    }
}

#[component]
pub fn OrgRepos() -> impl IntoView {
    let params = use_params_map();
    let org = move || params.with(|params| params.get("org").cloned().unwrap_or_default());

    let repos = create_resource(org, |org_name| async move {
        Request::get(&format!("http://127.0.0.1:3000/api/v1/orgs/{}/repos", org_name)).send().await.unwrap().json::<Vec<Repository>>().await.unwrap_or_default()
    });

    view! {
        <div class="org-repos">
            <h3>"Repositories"</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading..."</li> }>
                    {move || repos.get().map(|list| view! {
                        <For each=move || list.clone() key=|r| r.id children=move |r| {
                            let href = format!("/repos/{}/{}", r.owner, r.name);
                            view! { <li><a href=href>{r.name}</a></li> }
                        }/>
                    })}
                </Suspense>
            </ul>
        </div>
    }
}

#[component]
pub fn OrgTeams() -> impl IntoView {
    let params = use_params_map();
    let org = move || params.with(|params| params.get("org").cloned().unwrap_or_default());

    let teams = create_resource(org, |org_name| async move {
        Request::get(&format!("http://127.0.0.1:3000/api/v1/orgs/{}/teams", org_name)).send().await.unwrap().json::<Vec<Team>>().await.unwrap_or_default()
    });

    view! {
        <div class="org-teams">
            <h3>"Teams"</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading..."</li> }>
                    {move || teams.get().map(|list| view! {
                        <For each=move || list.clone() key=|t| t.id children=move |t| {
                            view! { <li>{t.name} " (" {t.permission} ")"</li> }
                        }/>
                    })}
                </Suspense>
            </ul>
        </div>
    }
}

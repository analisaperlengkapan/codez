use leptos::*;
use gloo_net::http::Request;
use leptos_router::*;
use shared::{Organization, OrgMember};

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
                    Some(Some(o)) => view! { <p>{o.description.unwrap_or_default()}</p> }.into_view(),
                    _ => view! { <p>"Org not found"</p> }.into_view()
                }}
            </Suspense>
            <OrgMembers/>
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
            <h3>"Members of " {org}</h3>
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

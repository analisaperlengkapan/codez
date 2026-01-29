use gloo_net::http::Request;
use leptos::*;
use leptos_router::*;
use shared::{CreateOrgOption, CreateTeamOption, OrgMember, Organization, Repository, Team};

#[component]
pub fn OrgProfile() -> impl IntoView {
    let params = use_params_map();
    let org_name = move || params.with(|params| params.get("org").cloned().unwrap_or_default());

    let (active_tab, set_active_tab) = create_signal("repos".to_string());
    let (refresh, set_refresh) = create_signal(0);

    let org = create_resource(org_name, |name| async move {
        Request::get(&format!("/api/v1/orgs/{}", name))
            .send()
            .await
            .unwrap()
            .json::<Option<Organization>>()
            .await
            .unwrap_or(None)
    });

    let repos = create_resource(
        move || (org_name(), active_tab.get(), refresh.get()),
        |(name, tab, _)| async move {
            if tab == "repos" {
                Request::get(&format!("/api/v1/orgs/{}/repos", name))
                    .send()
                    .await
                    .unwrap()
                    .json::<Vec<Repository>>()
                    .await
                    .unwrap_or_default()
            } else {
                vec![]
            }
        },
    );

    let teams = create_resource(
        move || (org_name(), active_tab.get(), refresh.get()),
        |(name, tab, _)| async move {
            if tab == "teams" {
                Request::get(&format!("/api/v1/orgs/{}/teams", name))
                    .send()
                    .await
                    .unwrap()
                    .json::<Vec<Team>>()
                    .await
                    .unwrap_or_default()
            } else {
                vec![]
            }
        },
    );

    let members = create_resource(
        move || (org_name(), active_tab.get(), refresh.get()),
        |(name, tab, _)| async move {
            if tab == "people" {
                Request::get(&format!("/api/v1/orgs/{}/members", name))
                    .send()
                    .await
                    .unwrap()
                    .json::<Vec<OrgMember>>()
                    .await
                    .unwrap_or_default()
            } else {
                vec![]
            }
        },
    );

    // Create Team Logic
    let (new_team_name, set_new_team_name) = create_signal("".to_string());
    let on_create_team = move |_| {
        let name = org_name();
        let payload = CreateTeamOption {
            name: new_team_name.get(),
            description: None,
            permission: "read".to_string(),
        };
        spawn_local(async move {
            let _ = Request::post(&format!("/api/v1/orgs/{}/teams", name))
                .json(&payload)
                .unwrap()
                .send()
                .await;
            set_new_team_name.set("".to_string());
            set_refresh.update(|n| *n += 1);
        });
    };

    view! {
        <div class="org-profile">
            <Suspense fallback=move || view! { <h3>"Loading..."</h3> }>
                {move || match org.get() {
                    Some(Some(o)) => view! {
                        <div class="org-header">
                            <h2>{o.username}</h2>
                            <p>{o.description.unwrap_or_default()}</p>
                        </div>
                        <div class="org-tabs" style="margin-top: 20px; border-bottom: 1px solid #ccc; padding-bottom: 5px;">
                            <button on:click=move |_| set_active_tab.set("repos".to_string())>
                                "Repositories"
                            </button>
                            <button on:click=move |_| set_active_tab.set("people".to_string())>
                                "People"
                            </button>
                            <button on:click=move |_| set_active_tab.set("teams".to_string())>
                                "Teams"
                            </button>
                        </div>
                        <div class="org-content" style="margin-top: 20px;">
                            {move || match active_tab.get().as_str() {
                                "repos" => view! {
                                    <ul>
                                        <Suspense fallback=move || view! { <li>"Loading repos..."</li> }>
                                            {move || repos.get().map(|list| view! {
                                                <For each=move || list.clone() key=|r| r.id children=move |r| {
                                                    let href = format!("/repos/{}/{}", r.owner, r.name);
                                                    view! { <li><a href=href>{r.name}</a></li> }
                                                }/>
                                            })}
                                        </Suspense>
                                    </ul>
                                }.into_view(),
                                "people" => view! {
                                    <ul>
                                        <Suspense fallback=move || view! { <li>"Loading members..."</li> }>
                                            {move || members.get().map(|list| view! {
                                                <For each=move || list.clone() key=|m| m.user.id children=move |m| {
                                                    view! { <li>{m.user.username} " (" {m.role} ")"</li> }
                                                }/>
                                            })}
                                        </Suspense>
                                    </ul>
                                }.into_view(),
                                "teams" => view! {
                                    <div>
                                        <ul>
                                            <Suspense fallback=move || view! { <li>"Loading teams..."</li> }>
                                                {move || teams.get().map(|list| view! {
                                                    <For each=move || list.clone() key=|t| t.id children=move |t| {
                                                        view! { <li>{t.name} " (" {t.permission} ")"</li> }
                                                    }/>
                                                })}
                                            </Suspense>
                                        </ul>
                                        <div class="create-team" style="margin-top: 10px;">
                                            <input type="text" placeholder="New Team Name" prop:value=new_team_name on:input=move |ev| set_new_team_name.set(event_target_value(&ev)) />
                                            <button on:click=on_create_team>"Create Team"</button>
                                        </div>
                                    </div>
                                }.into_view(),
                                _ => view! { <div></div> }.into_view()
                            }}
                        </div>
                    }.into_view(),
                    _ => view! { <h3>"Organization Not Found"</h3> }.into_view()
                }}
            </Suspense>
        </div>
    }
}

#[component]
pub fn CreateOrg() -> impl IntoView {
    let (name, set_name) = create_signal("".to_string());
    let (desc, set_desc) = create_signal("".to_string());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = CreateOrgOption {
            username: name.get(),
            description: if desc.get().is_empty() {
                None
            } else {
                Some(desc.get())
            },
            website: None,
            location: None,
            email: None,
            visibility: Some("public".to_string()),
        };
        spawn_local(async move {
            let _ = Request::post("/api/v1/orgs")
                .json(&payload)
                .unwrap()
                .send()
                .await;
            // Redirect to org profile?
        });
    };

    view! {
        <div class="create-org">
            <h3>"New Organization"</h3>
            <form on:submit=on_submit>
                <input type="text" placeholder="Organization Name" prop:value=name on:input=move |ev| set_name.set(event_target_value(&ev)) />
                <input type="text" placeholder="Description" prop:value=desc on:input=move |ev| set_desc.set(event_target_value(&ev)) />
                <button type="submit">"Create Organization"</button>
            </form>
        </div>
    }
}

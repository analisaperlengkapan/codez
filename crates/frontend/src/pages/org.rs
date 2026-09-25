use crate::api::{get, get_or, post_json, put_json};
use leptos::*;
use leptos_router::*;
use shared::{
    AuditLog, CreateOrgOption, CreateTeamOption, OrgMember, Organization, Repository, Team,
    UpdateMemberRoleOption,
};

#[component]
pub fn OrgProfile() -> impl IntoView {
    let params = use_params_map();
    let org_name = move || params.with(|params| params.get("org").cloned().unwrap_or_default());

    let (active_tab, set_active_tab) = create_signal("repos".to_string());
    let (refresh, set_refresh) = create_signal(0);

    let org = create_resource(org_name, |name| async move {
        get_or::<Option<Organization>>(&format!("/api/v1/orgs/{}", name), None).await
    });

    let repos = create_resource(
        move || (org_name(), active_tab.get(), refresh.get()),
        |(name, tab, _)| async move {
            if tab == "repos" {
                get::<Vec<Repository>>(&format!("/api/v1/orgs/{}/repos", name)).await
            } else {
                vec![]
            }
        },
    );

    let teams = create_resource(
        move || (org_name(), active_tab.get(), refresh.get()),
        |(name, tab, _)| async move {
            if tab == "teams" {
                get::<Vec<Team>>(&format!("/api/v1/orgs/{}/teams", name)).await
            } else {
                vec![]
            }
        },
    );

    let members = create_resource(
        move || (org_name(), active_tab.get(), refresh.get()),
        |(name, tab, _)| async move {
            if tab == "people" {
                get::<Vec<OrgMember>>(&format!("/api/v1/orgs/{}/members", name)).await
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
            let _ = post_json(&format!("/api/v1/orgs/{}/teams", name), &payload).await;
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
                        <div class="org-tabs">
                            <button on:click=move |_| set_active_tab.set("repos".to_string())>
                                "Repositories"
                            </button>
                            <button on:click=move |_| set_active_tab.set("people".to_string())>
                                "People"
                            </button>
                            <button on:click=move |_| set_active_tab.set("teams".to_string())>
                                "Teams"
                            </button>
                            <button on:click=move |_| set_active_tab.set("audit".to_string())>
                                "📋 Audit Logs"
                            </button>
                        </div>
                        <div class="org-content mt-2">
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
                                    <div>
                                        <h3>"Members & Role Management"</h3>
                                        <ul class="list-reset">
                                            <Suspense fallback=move || view! { <li>"Loading members..."</li> }>
                                                {move || members.get().map(|list| view! {
                                                    <For each=move || list.clone() key=|m| m.user.id children=move |m| {
                                                        let username = m.user.username.clone();
                                                        let current_role = m.role.clone();
                                                        let on_change_role = move |ev: leptos::ev::Event| {
                                                            let new_role = event_target_value(&ev);
                                                            let org_n = org_name();
                                                            let user_n = username.clone();
                                                            let payload = UpdateMemberRoleOption { role: new_role };
                                                            spawn_local(async move {
                                                                let _ = put_json(&format!("/api/v1/orgs/{}/members/{}", org_n, user_n), &payload).await;
                                                                set_refresh.update(|n| *n += 1);
                                                            });
                                                        };
                                                        view! {
                                                            <li class="flex-between list-row">
                                                                <span><strong>{m.user.username.clone()}</strong></span>
                                                                <select on:change=on_change_role prop:value=current_role>
                                                                    <option value="owner">"Owner"</option>
                                                                    <option value="maintainer">"Maintainer"</option>
                                                                    <option value="developer">"Developer"</option>
                                                                    <option value="reporter">"Reporter"</option>
                                                                    <option value="guest">"Guest"</option>
                                                                </select>
                                                            </li>
                                                        }
                                                    }/>
                                                })}
                                            </Suspense>
                                        </ul>
                                    </div>
                                }.into_view(),
                                "audit" => view! {
                                    <OrgAuditLogs org_name=org_name() />
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
                                        <div class="create-team mt-1">
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
pub fn OrgAuditLogs(org_name: String) -> impl IntoView {
    let logs = create_resource(
        move || org_name.clone(),
        |name| async move { get::<Vec<AuditLog>>(&format!("/api/v1/orgs/{}/audit-logs", name)).await },
    );

    view! {
        <div class="org-audit-logs">
            <h3>"Security Audit Trail Logs"</h3>
            <ul class="list-reset">
                <Suspense fallback=move || view! { <li>"Loading audit logs..."</li> }>
                    {move || logs.get().map(|list| view! {
                        <For each=move || list.clone() key=|l| l.id children=move |l| {
                            view! {
                                <li class="audit-log">
                                    <div class="flex-between">
                                        <strong class="text-accent">{l.action}</strong>
                                        <span class="text-small text-muted">{l.created_at}</span>
                                    </div>
                                    <p class="audit-details">{l.details}</p>
                                    <div class="text-small text-muted">
                                        "Actor: " <strong>{l.actor.username}</strong> " | Target: " {l.target_name}
                                    </div>
                                </li>
                            }
                        }/>
                    })}
                </Suspense>
            </ul>
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
            visibility: None,
        };
        spawn_local(async move {
            let _ = post_json("/api/v1/orgs", &payload).await;
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

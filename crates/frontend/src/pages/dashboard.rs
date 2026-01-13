use leptos::*;
use gloo_net::http::Request;
use shared::{Repository, Notification, Activity, Issue, PullRequest};

#[component]
pub fn UserDashboard() -> impl IntoView {
    let (active_tab, set_active_tab) = create_signal("feed".to_string());

    let repos = create_resource(|| (), |_| async move {
        Request::get("http://127.0.0.1:3000/api/v1/repos").send().await.unwrap().json::<Vec<Repository>>().await.unwrap_or_default()
    });

    let feeds = create_resource(|| (), |_| async move {
        Request::get("http://127.0.0.1:3000/api/v1/user/feeds").send().await.unwrap().json::<Vec<Activity>>().await.unwrap_or_default()
    });

    let assigned_issues = create_resource(
        move || active_tab.get(),
        |tab| async move {
            if tab == "issues" {
                Request::get("http://127.0.0.1:3000/api/v1/user/issues?state=open")
                    .send().await.unwrap().json::<Vec<Issue>>().await.unwrap_or_default()
            } else {
                vec![]
            }
        }
    );

    let my_pulls = create_resource(
        move || active_tab.get(),
        |tab| async move {
            if tab == "pulls" {
                Request::get("http://127.0.0.1:3000/api/v1/user/pulls?state=open")
                    .send().await.unwrap().json::<Vec<PullRequest>>().await.unwrap_or_default()
            } else {
                vec![]
            }
        }
    );

    view! {
        <div class="dashboard-container" style="display: flex;">
            <div class="dashboard-sidebar" style="width: 250px; padding-right: 20px;">
                <h3>"Repositories"</h3>
                <ul>
                    <Suspense fallback=move || view! { <li>"Loading repos..."</li> }>
                        {move || repos.get().map(|list| view! {
                            <For each=move || list.clone() key=|r| r.id children=move |r| {
                                let href = format!("/repos/{}/{}", r.owner, r.name);
                                view! { <li><a href=href>{r.owner} "/" {r.name}</a></li> }
                            }/>
                        })}
                    </Suspense>
                </ul>
            </div>
            <div class="dashboard-main" style="flex: 1;">
                <div class="dashboard-notifications">
                    <NotificationList/>
                </div>

                <div class="dashboard-tabs" style="margin-top: 20px; border-bottom: 1px solid #ccc; padding-bottom: 5px;">
                    <button
                        on:click=move |_| set_active_tab.set("feed".to_string())
                        style=move || if active_tab.get() == "feed" { "font-weight: bold; margin-right: 10px;" } else { "margin-right: 10px;" }
                    >
                        "Activity Feed"
                    </button>
                    <button
                        on:click=move |_| set_active_tab.set("issues".to_string())
                        style=move || if active_tab.get() == "issues" { "font-weight: bold; margin-right: 10px;" } else { "margin-right: 10px;" }
                    >
                        "My Issues"
                    </button>
                    <button
                        on:click=move |_| set_active_tab.set("pulls".to_string())
                        style=move || if active_tab.get() == "pulls" { "font-weight: bold;" } else { "" }
                    >
                        "My Pull Requests"
                    </button>
                </div>

                <div class="dashboard-content" style="margin-top: 20px;">
                    {move || match active_tab.get().as_str() {
                        "feed" => view! {
                            <div class="dashboard-feed">
                                <ul>
                                    <Suspense fallback=move || view! { <li>"Loading feed..."</li> }>
                                        {move || feeds.get().map(|list| view! {
                                            <For each=move || list.clone() key=|a| a.id children=move |a| {
                                                view! {
                                                    <li style="margin-bottom: 10px; border-bottom: 1px solid #eee; padding-bottom: 5px; display: flex; align-items: start;">
                                                        <div style="margin-right: 10px; font-size: 1.2em;">
                                                            {match a.op_type.as_str() {
                                                                "create_repo" => "📁",
                                                                "create_issue" => "🐛",
                                                                "create_pull_request" => "🔀",
                                                                _ => "📝"
                                                            }}
                                                        </div>
                                                        <div>
                                                            <div><strong>{a.user_name}</strong> " " {a.op_type}</div>
                                                            <div style="color: #666;">{a.content}</div>
                                                            <div style="font-size: 0.8em; color: #999;">{a.created}</div>
                                                        </div>
                                                    </li>
                                                }
                                            }/>
                                        })}
                                    </Suspense>
                                </ul>
                            </div>
                        }.into_view(),
                        "issues" => view! {
                            <div class="dashboard-issues">
                                <ul>
                                    <Suspense fallback=move || view! { <li>"Loading issues..."</li> }>
                                        {move || assigned_issues.get().map(|list| {
                                            if list.is_empty() {
                                                view! { <li>"No assigned issues found."</li> }.into_view()
                                            } else {
                                                view! {
                                                    <For each=move || list.clone() key=|i| i.id children=move |i| {
                                                        // Note: We need repo details to link correctly. For now assuming global lookups work or simple display.
                                                        // Since we don't have repo name in Issue struct, we can't easily link to /repos/:owner/:repo/issues/:id
                                                        // without fetching repo info.
                                                        // For this MVP, we'll display ID and Title.
                                                        view! {
                                                            <li>
                                                                <span>"Issue #" {i.number} ": " {i.title}</span>
                                                                <span style="margin-left: 10px; font-size: 0.8em; color: #666;">" (" {i.state} ")"</span>
                                                            </li>
                                                        }
                                                    }/>
                                                }.into_view()
                                            }
                                        })}
                                    </Suspense>
                                </ul>
                            </div>
                        }.into_view(),
                        "pulls" => view! {
                            <div class="dashboard-pulls">
                                <ul>
                                    <Suspense fallback=move || view! { <li>"Loading pull requests..."</li> }>
                                        {move || my_pulls.get().map(|list| {
                                            if list.is_empty() {
                                                view! { <li>"No pull requests found."</li> }.into_view()
                                            } else {
                                                view! {
                                                    <For each=move || list.clone() key=|p| p.id children=move |p| {
                                                        view! {
                                                            <li>
                                                                <span>"PR #" {p.number} ": " {p.title}</span>
                                                                <span style="margin-left: 10px; font-size: 0.8em; color: #666;">" (" {p.state} ")"</span>
                                                            </li>
                                                        }
                                                    }/>
                                                }.into_view()
                                            }
                                        })}
                                    </Suspense>
                                </ul>
                            </div>
                        }.into_view(),
                        _ => view! { <div></div> }.into_view()
                    }}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Explore() -> impl IntoView {
    let repos = create_resource(|| (), |_| async move {
        Request::get("http://127.0.0.1:3000/api/v1/repos").send().await.unwrap().json::<Vec<Repository>>().await.unwrap_or_default()
    });

    view! {
        <div class="explore">
            <h2>"Explore Codeza"</h2>
            <Search/>
            <h3>"Recent Repositories"</h3>
            <div class="explore-list">
                <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                    {move || repos.get().map(|list| view! {
                        <For each=move || list.clone() key=|r| r.id children=move |r| {
                            let href = format!("/repos/{}/{}", r.owner, r.name);
                            view! {
                                <div class="explore-item">
                                    <a href=href><strong>{r.owner} "/" {r.name}</strong></a>
                                    <p>{r.description.unwrap_or_default()}</p>
                                    <span>"⭐ " {r.stars_count}</span>
                                </div>
                            }
                        }/>
                    })}
                </Suspense>
            </div>
        </div>
    }
}

#[component]
pub fn Search() -> impl IntoView {
    let (query, set_query) = create_signal("".to_string());
    let (results, set_results) = create_signal(vec![]);

    let on_search = move |_| {
        let q = query.get();
        if !q.is_empty() {
            spawn_local(async move {
                let res = Request::get("http://127.0.0.1:3000/api/v1/repos/search").send().await.unwrap().json::<Vec<Repository>>().await.unwrap_or_default();
                set_results.set(res);
            });
        }
    };

    view! {
        <div class="search-page">
            <h2>"Search Repositories"</h2>
            <input type="text" placeholder="Search..."
                prop:value=query
                on:input=move |ev| set_query.set(event_target_value(&ev))
            />
            <button on:click=on_search>"Search"</button>
            <ul>
                <For each=move || results.get() key=|r| r.id children=move |r| {
                    view! { <li>{r.owner} "/" {r.name}</li> }
                }/>
            </ul>
        </div>
    }
}

#[component]
pub fn NotificationList() -> impl IntoView {
    let notifs = create_resource(|| (), |_| async move {
        Request::get("http://127.0.0.1:3000/api/v1/notifications").send().await.unwrap().json::<Vec<Notification>>().await.unwrap_or_default()
    });

    let on_mark_read = move |id: u64| {
        spawn_local(async move {
            let _ = Request::patch(&format!("http://127.0.0.1:3000/api/v1/notifications/threads/{}", id)).send().await;
        });
    };

    view! {
        <div class="notifications">
            <h2>"Notifications"</h2>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading..."</li> }>
                    {move || notifs.get().map(|list| view! {
                        <For each=move || list.clone() key=|n| n.id children=move |n| {
                            view! {
                                <li>
                                    <strong>{n.subject}</strong> " (" {if n.unread { "Unread" } else { "Read" }} ")"
                                    {if n.unread {
                                        view! { <button on:click=move |_| on_mark_read(n.id)>"Mark Read"</button> }.into_view()
                                    } else {
                                        view! { <span></span> }.into_view()
                                    }}
                                </li>
                            }
                        }/>
                    })}
                </Suspense>
            </ul>
        </div>
    }
}

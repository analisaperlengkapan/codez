use gloo_net::http::Request;
use leptos::*;
use shared::{Activity, Issue, Notification, PullRequest, Repository};

#[component]
pub fn UserDashboard() -> impl IntoView {
    let (active_tab, set_active_tab) = create_signal("feed".to_string());

    let repos = create_resource(
        || (),
        |_| async move {
            Request::get("/api/v1/repos")
                .send()
                .await
                .unwrap()
                .json::<Vec<Repository>>()
                .await
                .unwrap_or_default()
        },
    );

    let feeds = create_resource(
        || (),
        |_| async move {
            Request::get("/api/v1/user/feeds")
                .send()
                .await
                .unwrap()
                .json::<Vec<Activity>>()
                .await
                .unwrap_or_default()
        },
    );

    let assigned_issues = create_resource(
        move || active_tab.get(),
        |tab| async move {
            if tab == "issues" {
                Request::get("/api/v1/user/issues?state=open")
                    .send()
                    .await
                    .unwrap()
                    .json::<Vec<Issue>>()
                    .await
                    .unwrap_or_default()
            } else {
                vec![]
            }
        },
    );

    let my_pulls = create_resource(
        move || active_tab.get(),
        |tab| async move {
            if tab == "pulls" {
                Request::get("/api/v1/user/pulls?state=open")
                    .send()
                    .await
                    .unwrap()
                    .json::<Vec<PullRequest>>()
                    .await
                    .unwrap_or_default()
            } else {
                vec![]
            }
        },
    );

    view! {
        <div class="dashboard">
            <div class="page-header">
                <h2>"Dashboard"</h2>
                <div class="flex gap-sm">
                    <a class="btn" href="/repo/create">"New repository"</a>
                    <a class="btn" href="/org/create">"New organization"</a>
                </div>
            </div>

            <div class="repo-shell">
                <aside class="repo-sidebar">
                    <div class="panel">
                        <h3>"Your repositories"</h3>
                        <ul class="item-list">
                            <Suspense fallback=move || view! { <li class="text-muted">"Loading repositories…"</li> }>
                                {move || repos.get().map(|list| view! {
                                    <For each=move || list.clone() key=|r| r.id children=move |r| {
                                        let href = format!("/repos/{}/{}", r.owner, r.name);
                                        view! {
                                            <li>
                                                <a href=href><strong>{r.owner} "/" {r.name}</strong></a>
                                                <div class="text-small text-muted">{r.description.clone().unwrap_or_default()}</div>
                                            </li>
                                        }
                                    }/>
                                })}
                            </Suspense>
                        </ul>
                    </div>
                </aside>

                <div class="repo-main">
                    <NotificationList/>

                    <div class="tabs mt-2">
                        <button on:click=move |_| set_active_tab.set("feed".to_string()) class:active=move || active_tab.get() == "feed">"Activity feed"</button>
                        <button on:click=move |_| set_active_tab.set("issues".to_string()) class:active=move || active_tab.get() == "issues">"My issues"</button>
                        <button on:click=move |_| set_active_tab.set("pulls".to_string()) class:active=move || active_tab.get() == "pulls">"My pull requests"</button>
                    </div>

                    <div class="dashboard-content">
                        {move || match active_tab.get().as_str() {
                            "feed" => view! {
                                <ul class="item-list">
                                    <Suspense fallback=move || view! { <li class="text-muted">"Loading feed…"</li> }>
                                        {move || feeds.get().map(|list| {
                                            if list.is_empty() {
                                                view! { <li class="text-muted">"No recent activity."</li> }.into_view()
                                            } else {
                                                view! {
                                                    <For each=move || list.clone() key=|a| a.id children=move |a| {
                                                        view! {
                                                            <li class="flex-center">
                                                                <span class="text-2xl">
                                                                    {match a.op_type.as_str() {
                                                                        "create_repo" => "📁",
                                                                        "create_issue" => "🐛",
                                                                        "create_pull_request" => "🔀",
                                                                        _ => "📝"
                                                                    }}
                                                                </span>
                                                                <div>
                                                                    <div><strong>{a.user_name}</strong> " " {a.op_type.replace('_', " ")}</div>
                                                                    <div class="text-muted">{a.content}</div>
                                                                    <div class="text-small text-muted">{a.created}</div>
                                                                </div>
                                                            </li>
                                                        }
                                                    }/>
                                                }.into_view()
                                            }
                                        })}
                                    </Suspense>
                                </ul>
                            }.into_view(),
                            "issues" => view! {
                                <ul class="item-list">
                                    <Suspense fallback=move || view! { <li class="text-muted">"Loading issues…"</li> }>
                                        {move || assigned_issues.get().map(|list| {
                                            if list.is_empty() {
                                                view! { <li class="text-muted">"No assigned issues."</li> }.into_view()
                                            } else {
                                                view! {
                                                    <For each=move || list.clone() key=|i| i.id children=move |i| {
                                                        view! {
                                                            <li>
                                                                <span>"Issue #" {i.number} ": " {i.title}</span>
                                                                <span class="label">{i.state.clone()}</span>
                                                            </li>
                                                        }
                                                    }/>
                                                }.into_view()
                                            }
                                        })}
                                    </Suspense>
                                </ul>
                            }.into_view(),
                            "pulls" => view! {
                                <ul class="item-list">
                                    <Suspense fallback=move || view! { <li class="text-muted">"Loading pull requests…"</li> }>
                                        {move || my_pulls.get().map(|list| {
                                            if list.is_empty() {
                                                view! { <li class="text-muted">"No pull requests."</li> }.into_view()
                                            } else {
                                                view! {
                                                    <For each=move || list.clone() key=|p| p.id children=move |p| {
                                                        view! {
                                                            <li>
                                                                <span>"PR #" {p.number} ": " {p.title}</span>
                                                                <span class="label">{p.state.clone()}</span>
                                                            </li>
                                                        }
                                                    }/>
                                                }.into_view()
                                            }
                                        })}
                                    </Suspense>
                                </ul>
                            }.into_view(),
                            _ => view! { <div></div> }.into_view()
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Explore() -> impl IntoView {
    let repos = create_resource(
        || (),
        |_| async move {
            Request::get("/api/v1/repos")
                .send()
                .await
                .unwrap()
                .json::<Vec<Repository>>()
                .await
                .unwrap_or_default()
        },
    );

    view! {
        <div class="explore">
            <div class="page-header">
                <h2>"Explore Codeza"</h2>
            </div>
            <Search/>
            <h3 class="mt-3">"Recent repositories"</h3>
            <div class="explore-list">
                <Suspense fallback=move || view! { <p class="text-muted">"Loading…"</p> }>
                    {move || repos.get().map(|list| {
                        if list.is_empty() {
                            view! { <div class="empty-state">"No repositories yet."</div> }.into_view()
                        } else {
                            view! {
                                <For each=move || list.clone() key=|r| r.id children=move |r| {
                                    let href = format!("/repos/{}/{}", r.owner, r.name);
                                    view! {
                                        <div class="card">
                                            <a href=href><strong>{r.owner} "/" {r.name}</strong></a>
                                            <p class="text-muted mb-0">{r.description.clone().unwrap_or_else(|| "No description".to_string())}</p>
                                            <span class="text-small text-muted">"⭐ " {r.stars_count} " · 🍴 " {r.forks_count}</span>
                                        </div>
                                    }
                                }/>
                            }.into_view()
                        }
                    })}
                </Suspense>
            </div>
        </div>
    }
}

#[component]
pub fn Search() -> impl IntoView {
    let (query, set_query) = create_signal("".to_string());
    let (search_type, set_search_type) = create_signal("repos".to_string()); // repos | issues

    let (repo_results, set_repo_results) = create_signal(vec![]);
    let (issue_results, set_issue_results) = create_signal(vec![]);

    let on_search = move |_| {
        let q = query.get();
        let t = search_type.get();

        spawn_local(async move {
            if t == "repos" {
                let url = if !q.is_empty() {
                    format!("/api/v1/repos?q={}", q)
                } else {
                    "/api/v1/repos".to_string()
                };

                let res = Request::get(&url)
                    .send()
                    .await
                    .unwrap()
                    .json::<Vec<Repository>>()
                    .await
                    .unwrap_or_default();
                set_repo_results.set(res);
                set_issue_results.set(vec![]);
            } else {
                let url = format!("/api/v1/search/issues?q={}", q);
                let res = Request::get(&url)
                    .send()
                    .await
                    .unwrap()
                    .json::<Vec<Issue>>()
                    .await
                    .unwrap_or_default();
                set_issue_results.set(res);
                set_repo_results.set(vec![]);
            }
        });
    };

    view! {
        <div class="search-page">
            <div class="panel">
                <div class="flex-center flex-wrap">
                    <input type="text" placeholder="Search…"
                        prop:value=query
                        on:input=move |ev| set_query.set(event_target_value(&ev))
                        class="search-input-flex" />
                    <select on:change=move |ev| set_search_type.set(event_target_value(&ev)) class="w-auto">
                        <option value="repos">"Repositories"</option>
                        <option value="issues">"Issues"</option>
                    </select>
                    <button class="btn-primary" on:click=on_search>"Search"</button>
                </div>
            </div>

            <div class="search-results mt-2">
                {move || if search_type.get() == "repos" {
                    view! {
                        <ul class="item-list panel">
                            <For each=move || repo_results.get() key=|r| r.id children=move |r| {
                                let href = format!("/repos/{}/{}", r.owner, r.name);
                                view! {
                                    <li>
                                        <a href=href><strong>{r.owner} "/" {r.name}</strong></a>
                                        <p class="mb-0 text-muted">{r.description.clone().unwrap_or_default()}</p>
                                        <small class="text-muted">"⭐ " {r.stars_count}</small>
                                    </li>
                                }
                            }/>
                        </ul>
                    }.into_view()
                } else {
                    view! {
                        <ul class="item-list panel">
                            <For each=move || issue_results.get() key=|i| i.id children=move |i| {
                                view! {
                                    <li>
                                        <span><strong>"#" {i.number}</strong> " " {i.title}</span>
                                        <span class="label">{i.state.clone()}</span>
                                        <p class="mb-0 text-muted">{i.body.clone().unwrap_or_default().chars().take(120).collect::<String>()}</p>
                                    </li>
                                }
                            }/>
                        </ul>
                    }.into_view()
                }}
            </div>
        </div>
    }
}

#[component]
pub fn NotificationList() -> impl IntoView {
    let (refresh, set_refresh) = create_signal(0);
    let notifs = create_resource(
        move || refresh.get(),
        |_| async move {
            Request::get("/api/v1/notifications")
                .send()
                .await
                .unwrap()
                .json::<Vec<Notification>>()
                .await
                .unwrap_or_default()
        },
    );

    let on_mark_read = move |id: u64| {
        spawn_local(async move {
            let _ = Request::patch(&format!("/api/v1/notifications/threads/{}", id))
                .send()
                .await;
            set_refresh.update(|n| *n += 1);
        });
    };

    view! {
        <div class="notifications panel">
            <div class="page-header">
                <h3 class="mb-0">"Notifications"</h3>
            </div>
            <ul class="item-list">
                <Suspense fallback=move || view! { <li class="text-muted">"Loading…"</li> }>
                    {move || notifs.get().map(|list| {
                        if list.is_empty() {
                            view! { <li class="text-muted">"You have no notifications."</li> }.into_view()
                        } else {
                            view! {
                                <For each=move || list.clone() key=|n| n.id children=move |n| {
                                    let unread = n.unread;
                                    view! {
                                        <li class="flex-between">
                                            <span>
                                                <strong>{n.subject.clone()}</strong>
                                                {if unread {
                                                    view! { <span class="label label-accent">" (Unread)"</span> }.into_view()
                                                } else {
                                                    view! { <span class="text-small text-muted">" (Read)"</span> }.into_view()
                                                }}
                                            </span>
                                            {if unread {
                                                view! { <button class="btn-sm" on:click=move |_| on_mark_read(n.id)>"Mark Read"</button> }.into_view()
                                            } else {
                                                view! { <span class="text-small text-muted">"Read"</span> }.into_view()
                                            }}
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

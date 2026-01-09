use leptos::*;
use gloo_net::http::Request;
use shared::{Repository, Notification, Activity};

#[component]
pub fn UserDashboard() -> impl IntoView {
    let repos = create_resource(|| (), |_| async move {
        Request::get("http://127.0.0.1:3000/api/v1/repos").send().await.unwrap().json::<Vec<Repository>>().await.unwrap_or_default()
    });

    let feeds = create_resource(|| (), |_| async move {
        Request::get("http://127.0.0.1:3000/api/v1/user/feeds").send().await.unwrap().json::<Vec<Activity>>().await.unwrap_or_default()
    });

    view! {
        <div class="dashboard-container">
            <div class="dashboard-sidebar">
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
            <div class="dashboard-feed">
                <h3>"Activity Feed"</h3>
                <ul>
                    <Suspense fallback=move || view! { <li>"Loading feed..."</li> }>
                        {move || feeds.get().map(|list| view! {
                            <For each=move || list.clone() key=|a| a.id children=move |a| {
                                view! {
                                    <li>
                                        <strong>{a.user_name}</strong> " " {a.op_type} " - " {a.content}
                                    </li>
                                }
                            }/>
                        })}
                    </Suspense>
                </ul>
            </div>
        </div>
    }
}

#[component]
pub fn Explore() -> impl IntoView {
    view! {
        <div class="explore">
            <h2>"Explore Codeza"</h2>
            <Search/>
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

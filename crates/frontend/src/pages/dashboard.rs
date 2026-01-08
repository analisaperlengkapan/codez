use leptos::*;
use gloo_net::http::Request;
use shared::{Repository, Notification};

#[component]
pub fn UserDashboard() -> impl IntoView {
    let (repos, _set_repos) = create_signal(vec![
        Repository::new(1, "my-repo".to_string(), "me".to_string()),
    ]);

    view! {
        <div class="dashboard">
            <h2>"Dashboard"</h2>
            <ul>
                <For
                    each=move || repos.get()
                    key=|r| r.id
                    children=move |r| view! { <li>{r.name}</li> }
                />
            </ul>
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
                                </li>
                            }
                        }/>
                    })}
                </Suspense>
            </ul>
        </div>
    }
}

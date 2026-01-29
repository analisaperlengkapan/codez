use leptos::*;
use gloo_net::http::Request;
use shared::{AdminStats, User, SystemNotice};

#[component]
pub fn AdminDashboard() -> impl IntoView {
    let stats = create_resource(|| (), |_| async move {
        Request::get("http://127.0.0.1:3000/api/v1/admin/stats").send().await.unwrap().json::<AdminStats>().await.ok()
    });

    view! {
        <div class="admin-dashboard">
            <h2>"Admin Dashboard"</h2>
            <p><a href="/admin/users">"Manage Users"</a></p>
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || match stats.get() {
                    Some(Some(s)) => view! {
                        <div>
                            <p>"Users: " {s.users}</p>
                            <p>"Repos: " {s.repos}</p>
                            <p>"Orgs: " {s.orgs}</p>
                            <p>"Issues: " {s.issues}</p>
                        </div>
                    }.into_view(),
                    _ => view! { <p>"No stats"</p> }.into_view()
                }}
            </Suspense>
            <AdminNotices/>
        </div>
    }
}

#[component]
pub fn AdminUsers() -> impl IntoView {
    let users = create_resource(|| (), |_| async move {
        Request::get("http://127.0.0.1:3000/api/v1/admin/users").send().await.unwrap().json::<Vec<User>>().await.unwrap_or_default()
    });

    let on_delete = move |username: String| {
        spawn_local(async move {
            let _ = Request::delete(&format!("http://127.0.0.1:3000/api/v1/admin/users/{}", username)).send().await;
            // ideally refetch users here
        });
    };

    view! {
        <div class="admin-users">
            <h3>"User Management"</h3>
            <table>
                <thead><tr><th>"ID"</th><th>"Username"</th><th>"Email"</th><th>"Actions"</th></tr></thead>
                <tbody>
                    <Suspense fallback=move || view! { <tr><td colspan="4">"Loading..."</td></tr> }>
                        {move || users.get().map(|list| view! {
                            <For each=move || list.clone() key=|u| u.id children=move |u| {
                                let uname = u.username.clone();
                                view! {
                                    <tr>
                                        <td>{u.id}</td>
                                        <td>{u.username}</td>
                                        <td>{u.email}</td>
                                        <td>
                                            <button on:click=move |_| {
                                                let u = uname.clone();
                                                on_delete(u);
                                            }>"Delete"</button>
                                        </td>
                                    </tr>
                                }
                            }/>
                        })}
                    </Suspense>
                </tbody>
            </table>
        </div>
    }
}

#[component]
pub fn AdminNotices() -> impl IntoView {
    let notices = create_resource(|| (), |_| async move {
        Request::get("http://127.0.0.1:3000/api/v1/admin/notices").send().await.unwrap().json::<Vec<SystemNotice>>().await.unwrap_or_default()
    });

    view! {
        <div class="admin-notices">
            <h3>"System Notices"</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading..."</li> }>
                    {move || notices.get().map(|list| view! {
                        <For each=move || list.clone() key=|n| n.id children=move |n| {
                            view! { <li>[{n.type_}] {n.description}</li> }
                        }/>
                    })}
                </Suspense>
            </ul>
        </div>
    }
}

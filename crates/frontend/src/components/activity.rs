use leptos::*;
use gloo_net::http::Request;
use shared::Activity;

async fn fetch_activities() -> Vec<Activity> {
    Request::get("/api/v1/user/feeds").send().await.unwrap().json().await.unwrap_or_default()
}

#[component]
pub fn ActivityFeed() -> impl IntoView {
    let resource = create_resource(|| (), |_| async move { fetch_activities().await });

    view! {
        <div class="activity-feed">
            <h2>"Activity Feed"</h2>
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || {
                    resource.get().map(|activities| view! {
                        <ul>
                            <For each=move || activities.clone() key=|a| a.id children=move |a| {
                                view! { <li><strong>{a.user_name}</strong> " " {a.op_type} ": " {a.content}</li> }
                            }/>
                        </ul>
                    })
                }}
            </Suspense>
        </div>
    }
}

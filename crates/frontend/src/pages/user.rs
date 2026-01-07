use leptos::*;
use gloo_net::http::Request;
use shared::{User, LoginOption, RegisterOption, Contribution};
use leptos_router::*;

#[component]
pub fn Login() -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = LoginOption {
            username: username.get(),
            password: password.get(),
        };
        spawn_local(async move {
            let _ = Request::post("http://127.0.0.1:3000/api/v1/users/login").json(&payload).unwrap().send().await;
            leptos::logging::log!("Logged in");
        });
    };

    view! {
        <div class="login">
            <h2>"Login"</h2>
            <form on:submit=on_submit>
                <input type="text" placeholder="Username" prop:value=username on:input=move |ev| set_username.set(event_target_value(&ev)) />
                <input type="password" placeholder="Password" prop:value=password on:input=move |ev| set_password.set(event_target_value(&ev)) />
                <button type="submit">"Login"</button>
            </form>
        </div>
    }
}

#[component]
pub fn Register() -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());
    let (email, set_email) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = RegisterOption {
            username: username.get(),
            email: email.get(),
            password: password.get(),
        };
        spawn_local(async move {
            let _ = Request::post("http://127.0.0.1:3000/api/v1/users/register").json(&payload).unwrap().send().await;
            leptos::logging::log!("Registered");
        });
    };

    view! {
        <div class="register">
            <h2>"Register"</h2>
            <form on:submit=on_submit>
                <input type="text" placeholder="Username" prop:value=username on:input=move |ev| set_username.set(event_target_value(&ev)) />
                <input type="email" placeholder="Email" prop:value=email on:input=move |ev| set_email.set(event_target_value(&ev)) />
                <input type="password" placeholder="Password" prop:value=password on:input=move |ev| set_password.set(event_target_value(&ev)) />
                <button type="submit">"Register"</button>
            </form>
        </div>
    }
}

#[component]
pub fn UserProfile() -> impl IntoView {
    let params = use_params_map();
    let username = move || params.with(|params| params.get("username").cloned().unwrap_or_default());

    let user = create_resource(username, |u| async move {
        Request::get(&format!("http://127.0.0.1:3000/api/v1/users/{}", u)).send().await.unwrap().json::<Option<User>>().await.unwrap_or(None)
    });

    view! {
        <div class="user-profile">
            <h2>"User Profile: " {username}</h2>
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || match user.get() {
                    Some(Some(u)) => view! {
                        <div>
                            <p>"Email: " {u.email.unwrap_or("Hidden".to_string())}</p>
                            <UserHeatmap/>
                        </div>
                    }.into_view(),
                    _ => view! { <p>"User not found"</p> }.into_view()
                }}
            </Suspense>
        </div>
    }
}

#[component]
pub fn UserHeatmap() -> impl IntoView {
    let params = use_params_map();
    let username = move || params.with(|params| params.get("username").cloned().unwrap_or_default());
    let data = create_resource(username, |u| async move {
        Request::get(&format!("http://127.0.0.1:3000/api/v1/users/{}/heatmap", u)).send().await.unwrap().json::<Vec<Contribution>>().await.unwrap_or_default()
    });

    view! {
        <div class="user-heatmap">
            <h3>"Contributions"</h3>
            <div class="calendar-stub">
                <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                    {move || data.get().map(|list| view! {
                        <For each=move || list.clone() key=|c| c.date.clone() children=move |c| {
                             view! { <div title=format!("{} commits on {}", c.count, c.date) style="display:inline-block; width: 10px; height: 10px; background-color: green; margin: 1px;"></div> }
                        }/>
                    })}
                </Suspense>
            </div>
        </div>
    }
}

#[component]
pub fn UserFollowers() -> impl IntoView {
    let params = use_params_map();
    let username = move || params.with(|params| params.get("username").cloned().unwrap_or_default());
    let users = create_resource(username, |u| async move {
        Request::get(&format!("http://127.0.0.1:3000/api/v1/users/{}/followers", u)).send().await.unwrap().json::<Vec<User>>().await.unwrap_or_default()
    });

    view! {
        <div class="followers">
            <h3>"Followers"</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading..."</li> }>
                    {move || users.get().map(|list| view! {
                        <For each=move || list.clone() key=|u| u.id children=move |u| {
                            view! { <li>{u.username}</li> }
                        }/>
                    })}
                </Suspense>
            </ul>
        </div>
    }
}

#[component]
pub fn UserFollowing() -> impl IntoView {
    let params = use_params_map();
    let username = move || params.with(|params| params.get("username").cloned().unwrap_or_default());
    let users = create_resource(username, |u| async move {
        Request::get(&format!("http://127.0.0.1:3000/api/v1/users/{}/following", u)).send().await.unwrap().json::<Vec<User>>().await.unwrap_or_default()
    });

    view! {
        <div class="following">
            <h3>"Following"</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading..."</li> }>
                    {move || users.get().map(|list| view! {
                        <For each=move || list.clone() key=|u| u.id children=move |u| {
                            view! { <li>{u.username}</li> }
                        }/>
                    })}
                </Suspense>
            </ul>
        </div>
    }
}

#[component]
pub fn UserSettings() -> impl IntoView { view! { <div>"User Settings Placeholder"</div> } }

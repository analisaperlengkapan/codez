use gloo_net::http::Request;
use leptos::*;
use leptos_router::*;
use shared::{
    Contribution, GpgKey, LoginOption, PublicKey, RegisterOption, User, UserSettingsOption,
};

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
            let _ = Request::post("http://127.0.0.1:3000/api/v1/users/login")
                .json(&payload)
                .unwrap()
                .send()
                .await;
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
            let _ = Request::post("http://127.0.0.1:3000/api/v1/users/register")
                .json(&payload)
                .unwrap()
                .send()
                .await;
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
    let username =
        move || params.with(|params| params.get("username").cloned().unwrap_or_default());

    let user = create_resource(username, |u| async move {
        Request::get(&format!("http://127.0.0.1:3000/api/v1/users/{}", u))
            .send()
            .await
            .unwrap()
            .json::<Option<User>>()
            .await
            .unwrap_or(None)
    });

    view! {
        <div class="user-profile">
            <h2>"User Profile: " {username}</h2>
            <div class="user-links">
                <a href="followers">"Followers"</a> " | " <a href="following">"Following"</a>
            </div>
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
    let username =
        move || params.with(|params| params.get("username").cloned().unwrap_or_default());
    let data = create_resource(username, |u| async move {
        Request::get(&format!("http://127.0.0.1:3000/api/v1/users/{}/heatmap", u))
            .send()
            .await
            .unwrap()
            .json::<Vec<Contribution>>()
            .await
            .unwrap_or_default()
    });

    view! {
        <div class="user-heatmap">
            <h3>"Contributions"</h3>
            <div class="calendar-stub" style="display: grid; grid-template-columns: repeat(53, 1fr); gap: 2px;">
                <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                    {move || data.get().map(|list| view! {
                        <For each=move || list.clone() key=|c| c.date.clone() children=move |c| {
                             let color = if c.count == 0 { "#ebedf0" } else if c.count < 5 { "#9be9a8" } else { "#30a14e" };
                             view! { <div title=format!("{} commits on {}", c.count, c.date) style=format!("width: 10px; height: 10px; background-color: {};", color)></div> }
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
    let username =
        move || params.with(|params| params.get("username").cloned().unwrap_or_default());
    let users = create_resource(username, |u| async move {
        Request::get(&format!(
            "http://127.0.0.1:3000/api/v1/users/{}/followers",
            u
        ))
        .send()
        .await
        .unwrap()
        .json::<Vec<User>>()
        .await
        .unwrap_or_default()
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
    let username =
        move || params.with(|params| params.get("username").cloned().unwrap_or_default());
    let users = create_resource(username, |u| async move {
        Request::get(&format!(
            "http://127.0.0.1:3000/api/v1/users/{}/following",
            u
        ))
        .send()
        .await
        .unwrap()
        .json::<Vec<User>>()
        .await
        .unwrap_or_default()
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
pub fn UserSettings() -> impl IntoView {
    let settings = create_resource(
        || (),
        |_| async move {
            Request::get("http://127.0.0.1:3000/api/v1/user/settings")
                .send()
                .await
                .unwrap()
                .json::<UserSettingsOption>()
                .await
                .unwrap_or(UserSettingsOption {
                    full_name: None,
                    website: None,
                    description: None,
                    location: None,
                })
        },
    );

    let keys = create_resource(
        || (),
        |_| async move {
            Request::get("http://127.0.0.1:3000/api/v1/user/keys")
                .send()
                .await
                .unwrap()
                .json::<Vec<PublicKey>>()
                .await
                .unwrap_or_default()
        },
    );

    let gpg_keys = create_resource(
        || (),
        |_| async move {
            Request::get("http://127.0.0.1:3000/api/v1/user/gpg_keys")
                .send()
                .await
                .unwrap()
                .json::<Vec<GpgKey>>()
                .await
                .unwrap_or_default()
        },
    );

    let (full_name, set_full_name) = create_signal("".to_string());

    let on_update_profile = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = UserSettingsOption {
            full_name: Some(full_name.get()),
            website: None,
            description: None,
            location: None,
        };
        spawn_local(async move {
            let _ = Request::patch("http://127.0.0.1:3000/api/v1/user/settings")
                .json(&payload)
                .unwrap()
                .send()
                .await;
        });
    };

    view! {
        <div class="user-settings">
            <h2>"Settings"</h2>
            <div class="profile-settings">
                <h3>"Profile"</h3>
                <Suspense fallback=move || view! { <p>"Loading profile..."</p> }>
                    {move || settings.get().map(|s| view! {
                        <p>"Current Name: " {s.full_name.unwrap_or_default()}</p>
                    })}
                </Suspense>
                <form on:submit=on_update_profile>
                    <input type="text" placeholder="Full Name" prop:value=full_name on:input=move |ev| set_full_name.set(event_target_value(&ev)) />
                    <button type="submit">"Update Profile"</button>
                </form>
            </div>

            <div class="ssh-keys">
                <h3>"SSH Keys"</h3>
                <ul>
                    <Suspense fallback=move || view! { <li>"Loading..."</li> }>
                        {move || keys.get().map(|list| view! {
                            <For each=move || list.clone() key=|k| k.id children=move |k| {
                                view! { <li>{k.title} " - " {k.fingerprint}</li> }
                            }/>
                        })}
                    </Suspense>
                </ul>
            </div>

            <div class="gpg-keys">
                <h3>"GPG Keys"</h3>
                <ul>
                    <Suspense fallback=move || view! { <li>"Loading..."</li> }>
                        {move || gpg_keys.get().map(|list| view! {
                            <For each=move || list.clone() key=|k| k.id children=move |k| {
                                view! { <li>{k.key_id} " - " {k.primary_key_id}</li> }
                            }/>
                        })}
                    </Suspense>
                </ul>
            </div>
        </div>
    }
}

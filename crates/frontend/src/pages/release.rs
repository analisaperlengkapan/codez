use gloo_net::http::Request;
use leptos::*;
use leptos_router::*;
use shared::{CreateReleaseOption, Release};

#[component]
pub fn ReleaseList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let releases = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("/api/v1/repos/{}/{}/releases", o, r))
                .send()
                .await
                .unwrap()
                .json::<Vec<Release>>()
                .await
                .unwrap_or_default()
        },
    );

    view! {
        <div class="release-list">
            <div style="display: flex; justify-content: space-between; align-items: center;">
                <h3>"Releases for " {owner} "/" {repo_name}</h3>
                <a href=format!("/repos/{}/{}/releases/new", owner(), repo_name()) class="btn">"Draft a new release"</a>
            </div>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading releases..."</li> }>
                    {move || releases.get().map(|list| view! {
                        <For each=move || list.clone() key=|r| r.id children=move |r| {
                            let href = format!("/repos/{}/{}/releases/{}", owner(), repo_name(), r.id);
                            view! {
                                <li style="border-bottom: 1px solid #eee; padding: 10px 0;">
                                    <div style="display: flex; justify-content: space-between;">
                                        <div>
                                            <h4><a href=href>{r.name}</a></h4>
                                            <span style="background: #2cbe4e; color: white; padding: 2px 5px; border-radius: 3px; font-size: 0.8em;">{r.tag_name}</span>
                                            {if r.draft { view! { <span style="background: #6a737d; color: white; padding: 2px 5px; border-radius: 3px; font-size: 0.8em; margin-left: 5px;">"Draft"</span> }.into_view() } else { view! { <span></span> }.into_view() }}
                                            {if r.prerelease { view! { <span style="background: #dbab09; color: white; padding: 2px 5px; border-radius: 3px; font-size: 0.8em; margin-left: 5px;">"Pre-release"</span> }.into_view() } else { view! { <span></span> }.into_view() }}
                                        </div>
                                        <div style="color: #666; font-size: 0.9em;">
                                            {r.created_at}
                                        </div>
                                    </div>
                                    <p style="margin-top: 5px;">{r.body.unwrap_or_default()}</p>
                                    <div class="assets" style="margin-top: 10px;">
                                         <For each=move || r.assets.clone() key=|a| a.id children=move |a| {
                                            view! {
                                                <div style="font-size: 0.9em;">
                                                    <a href=a.download_url>"📦 " {a.name}</a> " (" {a.size} " bytes)"
                                                </div>
                                            }
                                         }/>
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
pub fn ReleaseDetail() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    // Trigger to refresh assets after upload
    let (trigger, set_trigger) = create_signal(0);

    let release = create_resource(
        move || (owner(), repo_name(), id(), trigger.get()),
        |(o, r, i, _)| async move {
            Request::get(&format!("/api/v1/repos/{}/{}/releases/{}", o, r, i))
                .send()
                .await
                .unwrap()
                .json::<Option<Release>>()
                .await
                .unwrap_or(None)
        },
    );

    let on_upload_asset = move |_| {
        let o = owner();
        let r = repo_name();
        let i = id();
        spawn_local(async move {
            let _ = Request::post(&format!("/api/v1/repos/{}/{}/releases/{}/assets", o, r, i))
                .send()
                .await;
            set_trigger.update(|n| *n += 1);
        });
    };

    let on_delete = move |_| {
        let o = owner();
        let r = repo_name();
        let i = id();
        spawn_local(async move {
            let _ = Request::delete(&format!("/api/v1/repos/{}/{}/releases/{}", o, r, i))
                .send()
                .await;
            // Redirect to list would be good here, simplistic mock for now
            let window = web_sys::window().unwrap();
            let _ = window
                .location()
                .set_href(&format!("/repos/{}/{}/releases", o, r));
        });
    };

    view! {
        <div class="release-detail">
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || match release.get() {
                    Some(Some(r)) => view! {
                        <div>
                            <div style="display: flex; justify-content: space-between; align-items: center;">
                                <h2>{r.name}</h2>
                                <div>
                                    <button on:click=on_delete style="background-color: #cb2431; color: white;">"Delete Release"</button>
                                </div>
                            </div>
                            <div class="meta" style="margin-bottom: 20px; border-bottom: 1px solid #eee; padding-bottom: 10px;">
                                <span style="font-weight: bold;">{r.tag_name}</span>
                                " | " {r.created_at} " | "
                                {if r.draft { "Draft" } else { "Published" }}
                            </div>
                            <div class="body" style="white-space: pre-wrap; margin-bottom: 20px;">
                                {r.body.unwrap_or_default()}
                            </div>
                            <div class="assets-section">
                                <h3>"Assets"</h3>
                                <ul>
                                    <For each=move || r.assets.clone() key=|a| a.id children=move |a| {
                                        view! {
                                            <li>
                                                <a href=a.download_url>"📦 " {a.name}</a> " (" {a.size} " bytes)"
                                            </li>
                                        }
                                    }/>
                                </ul>
                                <button on:click=on_upload_asset>"Upload New Asset (Mock)"</button>
                            </div>
                        </div>
                    }.into_view(),
                    _ => view! { <p>"Release not found"</p> }.into_view()
                }}
            </Suspense>
        </div>
    }
}

#[component]
pub fn ReleaseCreate() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (tag, set_tag) = create_signal("".to_string());
    let (name, set_name) = create_signal("".to_string());
    let (body, set_body) = create_signal("".to_string());
    let (draft, set_draft) = create_signal(false);
    let (prerelease, set_prerelease) = create_signal(false);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = CreateReleaseOption {
            tag_name: tag.get(),
            name: name.get(),
            body: Some(body.get()),
            draft: draft.get(),
            prerelease: prerelease.get(),
        };
        let o = owner();
        let r = repo_name();

        spawn_local(async move {
            let res = Request::post(&format!("/api/v1/repos/{}/{}/releases", o, r))
                .json(&payload)
                .unwrap()
                .send()
                .await;

            if res.is_ok() {
                // Redirect
                let window = web_sys::window().unwrap();
                let _ = window
                    .location()
                    .set_href(&format!("/repos/{}/{}/releases", o, r));
            }
        });
    };

    view! {
        <div class="release-create">
            <h3>"Create a new release"</h3>
            <form on:submit=on_submit>
                <div style="margin-bottom: 10px;">
                    <label style="display: block;">"Tag version"</label>
                    <input type="text" prop:value=tag on:input=move |ev| set_tag.set(event_target_value(&ev)) placeholder="v1.0.0" required style="width: 100%;"/>
                </div>
                <div style="margin-bottom: 10px;">
                    <label style="display: block;">"Release title"</label>
                    <input type="text" prop:value=name on:input=move |ev| set_name.set(event_target_value(&ev)) placeholder="Release title" required style="width: 100%;"/>
                </div>
                <div style="margin-bottom: 10px;">
                    <label style="display: block;">"Description"</label>
                    <textarea prop:value=body on:input=move |ev| set_body.set(event_target_value(&ev)) rows="10" style="width: 100%;"></textarea>
                </div>
                <div style="margin-bottom: 10px;">
                    <label>
                        <input type="checkbox" prop:checked=draft on:change=move |ev| set_draft.set(event_target_checked(&ev)) />
                        " This is a draft"
                    </label>
                </div>
                <div style="margin-bottom: 10px;">
                    <label>
                        <input type="checkbox" prop:checked=prerelease on:change=move |ev| set_prerelease.set(event_target_checked(&ev)) />
                        " This is a pre-release"
                    </label>
                </div>
                <button type="submit" style="background-color: #2cbe4e; color: white; padding: 10px 20px; border: none; border-radius: 5px; cursor: pointer;">"Publish Release"</button>
            </form>
        </div>
    }
}

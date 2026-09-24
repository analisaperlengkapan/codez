//! Repository creation and migration forms.

use gloo_net::http::Request;
use leptos::*;
use shared::{CreateRepoOption, GitignoreTemplate, LicenseTemplate};

#[component]
pub fn CreateRepo() -> impl IntoView {
    let (name, set_name) = create_signal("".to_string());
    let (desc, set_desc) = create_signal("".to_string());
    let (gitignore, set_gitignore) = create_signal("".to_string());
    let (license, set_license) = create_signal("".to_string());
    let (private, set_private) = create_signal(false);

    let licenses = create_resource(
        || (),
        |_| async move {
            Request::get("/api/v1/licenses")
                .send()
                .await
                .unwrap()
                .json::<Vec<LicenseTemplate>>()
                .await
                .unwrap_or_default()
        },
    );

    let gitignores = create_resource(
        || (),
        |_| async move {
            Request::get("/api/v1/gitignore/templates")
                .send()
                .await
                .unwrap()
                .json::<Vec<GitignoreTemplate>>()
                .await
                .unwrap_or_default()
        },
    );

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = CreateRepoOption {
            name: name.get(),
            description: if desc.get().is_empty() {
                None
            } else {
                Some(desc.get())
            },
            private: private.get(),
            auto_init: true,
            gitignores: if gitignore.get().is_empty() {
                None
            } else {
                Some(gitignore.get())
            },
            license: if license.get().is_empty() {
                None
            } else {
                Some(license.get())
            },
            readme: Some("Default".to_string()),
            default_branch: None,
            allow_rebase_merge: None,
            allow_squash_merge: None,
            allow_merge_commit: None,
            has_issues: None,
            has_wiki: None,
            has_projects: None,
        };
        spawn_local(async move {
            let _ = Request::post("/api/v1/user/repos")
                .json(&payload)
                .unwrap()
                .send()
                .await;
        });
    };

    view! {
        <div class="create-repo">
            <h3>"Create New Repository"</h3>
            <form on:submit=on_submit>
                <input type="text" placeholder="Repository Name" prop:value=name on:input=move |ev| set_name.set(event_target_value(&ev)) />
                <input type="text" placeholder="Description" prop:value=desc on:input=move |ev| set_desc.set(event_target_value(&ev)) />

                <div class="my-2">
                    <label>
                        <input type="checkbox" prop:checked=private on:change=move |ev| set_private.set(event_target_checked(&ev)) />
                        " Make Repository Private"
                    </label>
                </div>

                <Suspense fallback=move || view! { <select disabled><option>"Loading licenses..."</option></select> }>
                    {move || licenses.get().map(|list| view! {
                        <select on:change=move |ev| set_license.set(event_target_value(&ev))>
                            <option value="">"Select License"</option>
                            <For each=move || list.clone() key=|l| l.key.clone() children=move |l| {
                                view! { <option value={l.key}>{l.name}</option> }
                            }/>
                        </select>
                    })}
                </Suspense>

                <Suspense fallback=move || view! { <select disabled><option>"Loading gitignores..."</option></select> }>
                    {move || gitignores.get().map(|list| view! {
                        <select on:change=move |ev| set_gitignore.set(event_target_value(&ev))>
                            <option value="">"Select .gitignore"</option>
                            <For each=move || list.clone() key=|g| g.name.clone() children=move |g| {
                                view! { <option value={g.name.clone()}>{g.name}</option> }
                            }/>
                        </select>
                    })}
                </Suspense>

                <button type="submit">"Create"</button>
            </form>
            <p><a href="/repo/migrate">"Or Migrate Repository"</a></p>
        </div>
    }
}

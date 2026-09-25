//! Repository wiki pages.

use crate::api::{get, get_opt, get_or, post_json, put_json};
use crate::components::RepoNav;
use leptos::*;
use leptos_router::*;
use shared::{CreateWikiPageOption, WikiPage};

#[component]
pub fn Wiki() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());
    let page_name = move || {
        params.with(|params| {
            params
                .get("page_name")
                .cloned()
                .unwrap_or("Home".to_string())
        })
    };

    let wiki_page = create_resource(
        move || (owner(), repo_name(), page_name()),
        move |(o, r, p)| async move {
            get_or::<Option<WikiPage>>(&format!("/api/v1/repos/{}/{}/wiki/pages/{}", o, r, p), None)
                .await
        },
    );

    let wiki_pages = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            get::<Vec<WikiPage>>(&format!("/api/v1/repos/{}/{}/wiki/pages", o, r)).await
        },
    );

    view! {
        <div class="wiki-page">
            <RepoNav/>
            <div class="wiki-container flex">
                <div class="wiki-sidebar">
                    <h4>"Pages"</h4>
                    <ul>
                        <Suspense fallback=move || view! { <li>"Loading..."</li> }>
                            {move || wiki_pages.get().map(|list| view! {
                                <For each=move || list.clone() key=|p| p.title.clone() children=move |p| {
                                    let href = format!("/repos/{}/{}/wiki/pages/{}", owner(), repo_name(), p.title);
                                    view! { <li><a href=href>{p.title}</a></li> }
                                }/>
                            })}
                        </Suspense>
                    </ul>
                </div>
                <div class="wiki-view">
                    <Suspense fallback=move || view! { <p>"Loading wiki..."</p> }>
                        {move || match wiki_page.get() {
                            Some(Some(page)) => {
                                let title = page.title.clone();
                                view! {
                                    <div class="wiki-header">
                                        <h3>{page.title}</h3>
                                        <a href=format!("/repos/{}/{}/wiki/pages/{}/edit", owner(), repo_name(), title) class="btn">"Edit"</a>
                                    </div>
                                    <div class="wiki-content">
                                        <pre>{page.content}</pre>
                                    </div>
                                }.into_view()
                            },
                            _ => {
                                let p = page_name();
                                view! {
                                    <div>
                                        <p>"Wiki page '" {p.clone()} "' not found."</p>
                                        <a href=format!("/repos/{}/{}/wiki/pages/{}/edit", owner(), repo_name(), p)>
                                            "Create " {p} " Page"
                                        </a>
                                    </div>
                                }.into_view()
                            }
                        }}
                    </Suspense>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn WikiEdit() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());
    let page_name = move || {
        params.with(|params| {
            params
                .get("page_name")
                .cloned()
                .unwrap_or("Home".to_string())
        })
    };

    let (content, set_content) = create_signal("".to_string());
    let (message, set_message) = create_signal("".to_string());
    let (is_new, set_is_new) = create_signal(true);

    // Load existing content if available
    let _ = create_resource(
        move || (owner(), repo_name(), page_name()),
        move |(o, r, p)| async move {
            if let Some(Some(page)) =
                get_opt::<Option<WikiPage>>(&format!("/api/v1/repos/{}/{}/wiki/pages/{}", o, r, p))
                    .await
            {
                set_content.set(page.content);
                set_is_new.set(false);
            }
        },
    );

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = CreateWikiPageOption {
            title: page_name(),
            content: content.get(),
            message: Some(message.get()),
        };
        let o = owner();
        let r = repo_name();
        let p = page_name();
        let is_n = is_new.get();
        spawn_local(async move {
            if is_n {
                let _ = post_json(&format!("/api/v1/repos/{}/{}/wiki/pages", o, r), &payload).await;
            } else {
                let _ = put_json(
                    &format!("/api/v1/repos/{}/{}/wiki/pages/{}", o, r, p),
                    &payload,
                )
                .await;
            }
            // Redirect or notify would happen here
        });
    };

    view! {
        <div class="wiki-edit">
        <RepoNav/>
            <h3>"Editing " {page_name}</h3>
            <form on:submit=on_submit>
                <textarea prop:value=content on:input=move |ev| set_content.set(event_target_value(&ev)) rows="10"></textarea>
                <input type="text" placeholder="Commit Message" prop:value=message on:input=move |ev| set_message.set(event_target_value(&ev)) />
                <button type="submit">"Save Page"</button>
            </form>
        </div>
    }
}

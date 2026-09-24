use crate::components::RepoNav;
use gloo_net::http::Request;
use leptos::*;
use leptos_router::*;
use shared::{Branch, CreatePullRequestOption, PullRequest};

#[component]
pub fn CompareView() -> impl IntoView {
    let params = use_params_map();
    let navigate = use_navigate();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (base_branch, set_base_branch) = create_signal("main".to_string());
    let (compare_branch, set_compare_branch) = create_signal("".to_string());
    let (title, set_title) = create_signal("".to_string());
    let (body, set_body) = create_signal("".to_string());
    let (error_msg, set_error_msg) = create_signal(None::<String>);

    let branches = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("/api/v1/repos/{}/{}/branches", o, r))
                .send()
                .await
                .unwrap()
                .json::<Vec<Branch>>()
                .await
                .unwrap_or_default()
        },
    );

    // Set default compare branch when branches load
    create_effect(move |_| {
        if let Some(list) = branches.get() {
            if compare_branch.get().is_empty() && !list.is_empty() {
                // Default to the last branch that isn't main, or just the last one
                if let Some(b) = list.iter().find(|b| b.name != "main") {
                    set_compare_branch.set(b.name.clone());
                } else if let Some(first) = list.first() {
                    set_compare_branch.set(first.name.clone());
                }
            }
        }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = CreatePullRequestOption {
            title: title.get(),
            body: Some(body.get()),
            head: compare_branch.get(),
            base: base_branch.get(),
        };

        if payload.head == payload.base {
            set_error_msg.set(Some(
                "Base and Compare branches must be different.".to_string(),
            ));
            return;
        }

        let o = owner();
        let r = repo_name();
        let navigate = navigate.clone();
        spawn_local(async move {
            let res = Request::post(&format!("/api/v1/repos/{}/{}/pulls", o, r))
                .json(&payload)
                .unwrap()
                .send()
                .await;

            match res {
                Ok(resp) => {
                    if resp.ok() {
                        if let Ok(pr) = resp.json::<PullRequest>().await {
                            navigate(
                                &format!("/repos/{}/{}/pulls/{}", o, r, pr.number),
                                Default::default(),
                            );
                        }
                    } else {
                        set_error_msg.set(Some(
                            "Failed to create Pull Request. Ensure branches exist.".to_string(),
                        ));
                    }
                }
                Err(_) => {
                    set_error_msg.set(Some("Network error.".to_string()));
                }
            }
        });
    };

    view! {
        <div class="compare-view">
        <RepoNav/>
            <h3>"Compare changes"</h3>
            <p>"Choose two branches to see what’s changed or to start a new pull request."</p>

            <div class="branch-selector">
                 <Suspense fallback=move || view! { <span>"Loading branches..."</span> }>
                    {move || branches.get().map(|list| {
                        let list2 = list.clone();
                        view! {
                        <div class="flex-center gap-lg">
                            <div>
                                <label class="mr-1">"Base: "</label>
                                <select on:change=move |ev| set_base_branch.set(event_target_value(&ev))>
                                    <For each=move || list.clone() key=|b| b.name.clone() children=move |b| {
                                        let selected = b.name == base_branch.get();
                                        view! { <option value={b.name.clone()} selected={selected}>{b.name}</option> }
                                    }/>
                                </select>
                            </div>
                            <div>"←"</div>
                            <div>
                                <label class="mr-1">"Compare: "</label>
                                <select on:change=move |ev| set_compare_branch.set(event_target_value(&ev))>
                                    <For each=move || list2.clone() key=|b| b.name.clone() children=move |b| {
                                        let selected = b.name == compare_branch.get();
                                        view! { <option value={b.name.clone()} selected={selected}>{b.name}</option> }
                                    }/>
                                </select>
                            </div>
                        </div>
                    }})}
                </Suspense>
            </div>

            {move || if let Some(msg) = error_msg.get() {
                view! { <div class="error-msg">{msg}</div> }.into_view()
            } else {
                view! { <span></span> }.into_view()
            }}

            <form on:submit=on_submit class="pr-form">
                <div class="mb-1">
                    <input type="text" placeholder="Title" prop:value=title on:input=move |ev| set_title.set(event_target_value(&ev)) class="input-lg" required />
                </div>
                <div class="mb-1">
                    <textarea placeholder="Leave a comment" prop:value=body on:input=move |ev| set_body.set(event_target_value(&ev)) rows="10" class="p-1"></textarea>
                </div>
                <button type="submit" class="btn-primary">
                    "Create Pull Request"
                </button>
            </form>
        </div>
    }
}

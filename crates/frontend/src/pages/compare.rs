use leptos::*;
use leptos_router::*;
use gloo_net::http::Request;
use shared::{Branch, CreatePullRequestOption, PullRequest, Repository};

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

    let _repo_meta = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}", o, r))
                .send().await.unwrap().json::<Option<Repository>>().await.unwrap_or(None)
        }
    );

    let branches = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/branches", o, r))
                .send().await.unwrap().json::<Vec<Branch>>().await.unwrap_or_default()
        }
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
             set_error_msg.set(Some("Base and Compare branches must be different.".to_string()));
             return;
        }

        let o = owner();
        let r = repo_name();
        let navigate = navigate.clone();
        spawn_local(async move {
            let res = Request::post(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/pulls", o, r))
                .json(&payload).unwrap().send().await;

            match res {
                Ok(resp) => {
                    if resp.ok() {
                        if let Ok(pr) = resp.json::<PullRequest>().await {
                            navigate(&format!("/repos/{}/{}/pulls/{}", o, r, pr.number), Default::default());
                        }
                    } else {
                        set_error_msg.set(Some("Failed to create Pull Request. Ensure branches exist.".to_string()));
                    }
                },
                Err(_) => {
                    set_error_msg.set(Some("Network error.".to_string()));
                }
            }
        });
    };

    view! {
        <div class="compare-view">
            <h3>"Compare changes"</h3>
            <p>"Choose two branches to see what’s changed or to start a new pull request."</p>

            <div class="branch-selector" style="margin-bottom: 20px; padding: 10px; background: #f6f8fa; border: 1px solid #d0d7de; border-radius: 6px;">
                 <Suspense fallback=move || view! { <span>"Loading branches..."</span> }>
                    {move || branches.get().map(|list| {
                        let list2 = list.clone();
                        view! {
                        <div style="display: flex; gap: 20px; align-items: center;">
                            <div>
                                <label style="margin-right: 5px;">"Base: "</label>
                                <select on:change=move |ev| set_base_branch.set(event_target_value(&ev))>
                                    <For each=move || list.clone() key=|b| b.name.clone() children=move |b| {
                                        let selected = b.name == base_branch.get();
                                        view! { <option value={b.name.clone()} selected={selected}>{b.name}</option> }
                                    }/>
                                </select>
                            </div>
                            <div>"←"</div>
                            <div>
                                <label style="margin-right: 5px;">"Compare: "</label>
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
                view! { <div class="error-msg" style="color: red; margin-bottom: 10px;">{msg}</div> }.into_view()
            } else {
                view! { <span></span> }.into_view()
            }}

            <form on:submit=on_submit class="pr-form" style="border: 1px solid #d0d7de; padding: 20px; border-radius: 6px;">
                <div style="margin-bottom: 10px;">
                    <input type="text" placeholder="Title" prop:value=title on:input=move |ev| set_title.set(event_target_value(&ev)) style="width: 100%; font-size: 1.2em; padding: 5px;" required />
                </div>
                <div style="margin-bottom: 10px;">
                    <textarea placeholder="Leave a comment" prop:value=body on:input=move |ev| set_body.set(event_target_value(&ev)) rows="10" style="width: 100%; padding: 5px;"></textarea>
                </div>
                <button type="submit" class="btn-primary" style="background-color: #2da44e; color: white; padding: 5px 15px; border: none; border-radius: 6px; cursor: pointer;">
                    "Create Pull Request"
                </button>
            </form>
        </div>
    }
}

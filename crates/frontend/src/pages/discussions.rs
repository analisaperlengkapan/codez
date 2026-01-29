use leptos::*;
use gloo_net::http::Request;
use leptos_router::*;
use shared::{Discussion, CreateDiscussionOption, DiscussionComment, CreateDiscussionCommentOption};
use crate::api::api_url;

#[component]
pub fn DiscussionList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (show_create, set_show_create) = create_signal(false);
    let (refresh, set_refresh) = create_signal(0);
    let (new_title, set_new_title) = create_signal("".to_string());
    let (new_body, set_new_body) = create_signal("".to_string());
    let (new_category, set_new_category) = create_signal("General".to_string());

    let discussions = create_resource(
        move || (owner(), repo_name(), refresh.get()),
        |(o, r, _)| async move {
            let url = api_url(&format!("/repos/{}/{}/discussions", o, r));
            Request::get(&url)
                .send().await.unwrap().json::<Vec<Discussion>>().await.unwrap_or_default()
        }
    );

    let on_create = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = CreateDiscussionOption {
            title: new_title.get(),
            body: new_body.get(),
            category: new_category.get(),
        };
        let o = owner();
        let r = repo_name();
        spawn_local(async move {
            let url = api_url(&format!("/repos/{}/{}/discussions", o, r));
            let _ = Request::post(&url).json(&payload).unwrap().send().await;
            set_new_title.set("".to_string());
            set_new_body.set("".to_string());
            set_show_create.set(false);
            set_refresh.update(|n| *n += 1);
        });
    };

    view! {
        <div class="discussion-list">
            <div class="header" style="display: flex; justify-content: space-between; align-items: center;">
                <h3>"Discussions"</h3>
                <button on:click=move |_| set_show_create.set(!show_create.get())>
                    {move || if show_create.get() { "Cancel" } else { "New Discussion" }}
                </button>
            </div>

            {move || if show_create.get() {
                view! {
                    <form on:submit=on_create style="margin-bottom: 20px; padding: 10px; border: 1px solid #ccc;">
                        <input type="text" placeholder="Title" prop:value=new_title on:input=move |ev| set_new_title.set(event_target_value(&ev)) style="display: block; width: 100%; margin-bottom: 5px;" required />
                        <textarea placeholder="Body" prop:value=new_body on:input=move |ev| set_new_body.set(event_target_value(&ev)) style="display: block; width: 100%; margin-bottom: 5px;" rows="5"></textarea>
                        <select on:change=move |ev| set_new_category.set(event_target_value(&ev)) style="margin-bottom: 5px;">
                            <option value="General">"General"</option>
                            <option value="Ideas">"Ideas"</option>
                            <option value="Q&A">"Q&A"</option>
                            <option value="Show and Tell">"Show and Tell"</option>
                        </select>
                        <button type="submit">"Start Discussion"</button>
                    </form>
                }.into_view()
            } else {
                view! { <span></span> }.into_view()
            }}

            <ul>
                <Suspense fallback=move || view! { <li>"Loading discussions..."</li> }>
                    {move || discussions.get().map(|list| {
                        if list.is_empty() {
                            view! { <li>"No discussions found."</li> }.into_view()
                        } else {
                            view! {
                                <For each=move || list.clone() key=|d| d.id children=move |d| {
                                    let href = format!("/repos/{}/{}/discussions/{}", owner(), repo_name(), d.id);
                                    view! {
                                        <li style="margin-bottom: 10px; border: 1px solid #eee; padding: 10px;">
                                            <div style="font-size: 0.8em; color: #666;">{d.category}</div>
                                            <a href=href style="font-weight: bold; font-size: 1.1em;">{d.title}</a>
                                            <p style="margin: 5px 0;">{d.body}</p>
                                            <div style="font-size: 0.8em;">"by " {d.user.username}</div>
                                        </li>
                                    }
                                }/>
                            }.into_view()
                        }
                    })}
                </Suspense>
            </ul>
        </div>
    }
}

#[component]
pub fn DiscussionDetail() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default().parse::<u64>().unwrap_or_default());

    let (refresh_comments, set_refresh_comments) = create_signal(0);
    let (new_comment_body, set_new_comment_body) = create_signal("".to_string());

    let discussion = create_resource(
        move || (owner(), repo_name(), id()),
        |(o, r, i)| async move {
            let url = api_url(&format!("/repos/{}/{}/discussions/{}", o, r, i));
            Request::get(&url)
                .send().await.unwrap().json::<Option<Discussion>>().await.unwrap_or(None)
        }
    );

    let comments = create_resource(
        move || (owner(), repo_name(), id(), refresh_comments.get()),
        |(o, r, i, _)| async move {
            let url = api_url(&format!("/repos/{}/{}/discussions/{}/comments", o, r, i));
            Request::get(&url)
                .send().await.unwrap().json::<Vec<DiscussionComment>>().await.unwrap_or_default()
        }
    );

    let on_submit_comment = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = CreateDiscussionCommentOption {
            body: new_comment_body.get(),
        };
        let o = owner();
        let r = repo_name();
        let i = id();

        spawn_local(async move {
            let url = api_url(&format!("/repos/{}/{}/discussions/{}/comments", o, r, i));
            let res = Request::post(&url).json(&payload).unwrap().send().await;
            if let Ok(resp) = res {
                if resp.ok() {
                    set_new_comment_body.set("".to_string());
                    set_refresh_comments.update(|n| *n += 1);
                }
            }
        });
    };

    view! {
        <div class="discussion-detail">
            <Suspense fallback=move || view! { <h3>"Loading Discussion..."</h3> }>
                {move || discussion.get().map(|d| match d {
                    Some(disc) => {
                        view! {
                            <div class="discussion-header">
                                <span style="background: #eee; padding: 2px 5px; border-radius: 5px; font-size: 0.8em;">{disc.category}</span>
                                <h1>{disc.title}</h1>
                                <div class="meta">
                                    "Started by " <strong>{disc.user.username}</strong> " on " {disc.created_at}
                                </div>
                            </div>
                            <div class="discussion-body" style="margin-top: 20px; padding: 10px; border: 1px solid #ddd; border-radius: 5px;">
                                <p>{disc.body}</p>
                            </div>

                            <div class="discussion-comments" style="margin-top: 30px;">
                                <h3>"Comments"</h3>
                                <ul style="list-style: none; padding: 0;">
                                    <Suspense fallback=move || view! { <li>"Loading comments..."</li> }>
                                        {move || comments.get().map(|list| {
                                            view! {
                                                <For each=move || list.clone() key=|c| c.id children=move |c| {
                                                    view! {
                                                        <li style="margin-bottom: 15px; border-top: 1px solid #eee; padding-top: 10px;">
                                                            <div style="font-size: 0.9em; font-weight: bold;">{c.user.username} " commented:"</div>
                                                            <p style="margin: 5px 0;">{c.body}</p>
                                                            <div style="font-size: 0.8em; color: #888;">{c.created_at}</div>
                                                        </li>
                                                    }
                                                }/>
                                            }
                                        })}
                                    </Suspense>
                                </ul>

                                <form on:submit=on_submit_comment style="margin-top: 20px; border-top: 1px solid #ddd; padding-top: 10px;">
                                    <h4>"Add a Comment"</h4>
                                    <textarea
                                        prop:value=new_comment_body
                                        on:input=move |ev| set_new_comment_body.set(event_target_value(&ev))
                                        style="width: 100%; height: 100px; margin-bottom: 10px;"
                                        placeholder="Write your comment here..."
                                        required
                                    ></textarea>
                                    <button type="submit">"Post Comment"</button>
                                </form>
                            </div>
                        }.into_view()
                    },
                    None => view! { <h3>"Discussion Not Found"</h3> }.into_view()
                })}
            </Suspense>
        </div>
    }
}

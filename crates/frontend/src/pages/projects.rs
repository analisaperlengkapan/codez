use leptos::*;
use gloo_net::http::Request;
use leptos_router::*;
use shared::{
    Project, CreateProjectOption, ProjectColumn, CreateProjectColumnOption,
    ProjectCard, CreateProjectCardOption, Issue
};

#[component]
pub fn ProjectList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (show_create, set_show_create) = create_signal(false);
    let (new_title, set_new_title) = create_signal("".to_string());
    let (new_desc, set_new_desc) = create_signal("".to_string());

    let projects = create_resource(
        move || (owner(), repo_name(), show_create.get()), // refresh on create toggle/submit
        |(o, r, _)| async move {
            Request::get(&format!("/api/v1/repos/{}/{}/projects", o, r))
                .send().await.unwrap().json::<Vec<Project>>().await.unwrap_or_default()
        }
    );

    let on_create = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = CreateProjectOption {
            title: new_title.get(),
            description: if new_desc.get().is_empty() { None } else { Some(new_desc.get()) },
        };
        let o = owner();
        let r = repo_name();
        spawn_local(async move {
            let _ = Request::post(&format!("/api/v1/repos/{}/{}/projects", o, r))
                .json(&payload).unwrap().send().await;
            set_new_title.set("".to_string());
            set_new_desc.set("".to_string());
            set_show_create.set(false);
        });
    };

    view! {
        <div class="project-list">
            <div class="header" style="display: flex; justify-content: space-between; align-items: center;">
                <h3>"Projects"</h3>
                <button on:click=move |_| set_show_create.set(!show_create.get())>
                    {move || if show_create.get() { "Cancel" } else { "New Project" }}
                </button>
            </div>

            {move || if show_create.get() {
                view! {
                    <form on:submit=on_create style="margin-bottom: 20px; padding: 10px; border: 1px solid #ccc;">
                        <input type="text" placeholder="Project Title" prop:value=new_title on:input=move |ev| set_new_title.set(event_target_value(&ev)) style="display: block; width: 100%; margin-bottom: 5px;" required />
                        <textarea placeholder="Description" prop:value=new_desc on:input=move |ev| set_new_desc.set(event_target_value(&ev)) style="display: block; width: 100%; margin-bottom: 5px;"></textarea>
                        <button type="submit">"Create Project"</button>
                    </form>
                }.into_view()
            } else {
                view! { <span></span> }.into_view()
            }}

            <ul>
                <Suspense fallback=move || view! { <li>"Loading projects..."</li> }>
                    {move || projects.get().map(|list| {
                         if list.is_empty() {
                            view! { <li>"No projects found."</li> }.into_view()
                        } else {
                            view! {
                                <For each=move || list.clone() key=|p| p.id children=move |p| {
                                    let href = format!("/repos/{}/{}/projects/{}", owner(), repo_name(), p.id);
                                    view! {
                                        <li style="margin-bottom: 10px; border: 1px solid #eee; padding: 10px;">
                                            <a href=href style="font-weight: bold; font-size: 1.1em;">{p.title}</a>
                                            <p style="margin: 5px 0;">{p.description.unwrap_or_default()}</p>
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
pub fn ProjectDetail() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default().parse::<u64>().unwrap_or_default());

    let (refresh, set_refresh) = create_signal(0);

    let project = create_resource(
        move || (owner(), repo_name(), id()),
        |(o, r, i)| async move {
            Request::get(&format!("/api/v1/repos/{}/{}/projects/{}", o, r, i))
                .send().await.unwrap().json::<Option<Project>>().await.unwrap_or(None)
        }
    );

    let columns = create_resource(
        move || (owner(), repo_name(), id(), refresh.get()),
        |(o, r, i, _)| async move {
            Request::get(&format!("/api/v1/repos/{}/{}/projects/{}/columns", o, r, i))
                .send().await.unwrap().json::<Vec<ProjectColumn>>().await.unwrap_or_default()
        }
    );

    let (new_col_title, set_new_col_title) = create_signal("".to_string());

    let on_add_column = move |_| {
        let o = owner();
        let r = repo_name();
        let i = id();
        let payload = CreateProjectColumnOption { title: new_col_title.get() };
        if !payload.title.is_empty() {
            spawn_local(async move {
                let _ = Request::post(&format!("/api/v1/repos/{}/{}/projects/{}/columns", o, r, i))
                    .json(&payload).unwrap().send().await;
                set_new_col_title.set("".to_string());
                set_refresh.update(|n| *n += 1);
            });
        }
    };

    let on_toggle_close = move |is_closed: bool| {
        let o = owner();
        let r = repo_name();
        let i = id();
        let action = if is_closed { "reopen" } else { "close" };
        spawn_local(async move {
            let _ = Request::post(&format!("/api/v1/repos/{}/{}/projects/{}/{}", o, r, i, action))
                .send().await;
            set_refresh.update(|n| *n += 1); // Trigger resource reload
        });
    };

    view! {
        <div class="project-board" style="height: 100%; display: flex; flex-direction: column;">
            <Suspense fallback=move || view! { <h3>"Loading Project..."</h3> }>
                {move || project.get().map(|p| match p {
                    Some(proj) => {
                        let is_closed = proj.is_closed;
                        view! {
                            <div class="board-header" style="margin-bottom: 10px; display: flex; justify-content: space-between;">
                                <div>
                                    <h3>{proj.title} {if is_closed { " (Closed)" } else { "" }}</h3>
                                    <p>{proj.description.unwrap_or_default()}</p>
                                </div>
                                <button on:click=move |_| on_toggle_close(is_closed)>
                                    {if is_closed { "Reopen Project" } else { "Close Project" }}
                                </button>
                            </div>
                        }.into_view()
                    },
                    None => view! { <h3>"Project Not Found"</h3> }.into_view()
                })}
            </Suspense>

            <div class="board-columns" style="display: flex; overflow-x: auto; gap: 10px; padding-bottom: 20px; flex: 1;">
                 <Suspense fallback=move || view! { <div>"Loading columns..."</div> }>
                    {move || columns.get().map(|cols| view! {
                        <For each=move || cols.clone() key=|c| c.id children=move |c| {
                            view! { <ProjectColumnView column=c repo_owner=owner() repo_name=repo_name() _project_id=id() _refresh_signal=set_refresh /> }
                        }/>
                    })}
                </Suspense>

                <div class="add-column" style="min_width: 250px; background: #f0f0f0; padding: 10px; border-radius: 5px;">
                    <input type="text" placeholder="New Column" prop:value=new_col_title on:input=move |ev| set_new_col_title.set(event_target_value(&ev)) />
                    <button on:click=on_add_column>"Add Column"</button>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ProjectColumnView(
    column: ProjectColumn,
    repo_owner: String,
    repo_name: String,
    _project_id: u64,
    _refresh_signal: WriteSignal<i32>
) -> impl IntoView {
    let (refresh_cards, set_refresh_cards) = create_signal(0);
    let (new_card_content, set_new_card_content) = create_signal("".to_string());
    let (issue_id_input, set_issue_id_input) = create_signal("".to_string());

    let column_id = column.id;
    let o = repo_owner.clone();
    let r = repo_name.clone();

    let o_cards = o.clone();
    let r_cards = r.clone();
    let cards = create_resource(
        move || (o_cards.clone(), r_cards.clone(), column_id, refresh_cards.get()), // also depends on global refresh? No, local is enough unless moved
        move |(o, r, c, _)| async move {
            Request::get(&format!("/api/v1/repos/{}/{}/projects/columns/{}/cards", o, r, c))
                .send().await.unwrap().json::<Vec<ProjectCard>>().await.unwrap_or_default()
        }
    );

    let o_issues = o.clone();
    let r_issues = r.clone();
    let issues = create_resource(
        move || (o_issues.clone(), r_issues.clone()),
        move |(o, r)| async move {
            Request::get(&format!("/api/v1/repos/{}/{}/issues?state=open", o, r))
                .send().await.unwrap().json::<Vec<Issue>>().await.unwrap_or_default()
        }
    );

    let on_add_card = move |_| {
        let o = repo_owner.clone();
        let r = repo_name.clone();
        let c = column_id;

        let issue_id = issue_id_input.get().parse::<u64>().ok();
        let content = new_card_content.get();

        let payload = CreateProjectCardOption {
            content: if content.is_empty() { None } else { Some(content) },
            note: None,
            issue_id,
        };

        if payload.content.is_some() || payload.issue_id.is_some() {
             spawn_local(async move {
                let _ = Request::post(&format!("/api/v1/repos/{}/{}/projects/columns/{}/cards", o, r, c))
                    .json(&payload).unwrap().send().await;
                set_new_card_content.set("".to_string());
                set_issue_id_input.set("".to_string());
                set_refresh_cards.update(|n| *n += 1);
            });
        }
    };

    view! {
        <div class="column" style="min-width: 250px; max-width: 250px; background: #e0e0e0; padding: 10px; border-radius: 5px; display: flex; flex-direction: column;">
            <h4 style="margin-top: 0;">{column.title}</h4>
            <div class="cards" style="flex: 1; overflow-y: auto; display: flex; flex-direction: column; gap: 5px;">
                <Suspense fallback=move || view! { <div>"Loading..."</div> }>
                    {move || cards.get().map(|list| view! {
                        <For each=move || list.clone() key=|card| card.id children=move |card| {
                            let issue_link = card.issue_id.map(|id| format!("Issue #{}", id));
                            view! {
                                <div class="card" style="background: white; padding: 8px; border-radius: 3px; box-shadow: 0 1px 2px rgba(0,0,0,0.1);">
                                    {if let Some(link) = issue_link {
                                        view! { <div style="font-weight: bold; color: #0366d6;">{link}</div> }.into_view()
                                    } else {
                                        view! { <span></span> }.into_view()
                                    }}
                                    <div>{card.content.unwrap_or_default()}</div>
                                </div>
                            }
                        }/>
                    })}
                </Suspense>
            </div>
             <div class="add-card" style="margin-top: 10px;">
                <textarea placeholder="Card content..." prop:value=new_card_content on:input=move |ev| set_new_card_content.set(event_target_value(&ev)) style="width: 100%; box-sizing: border-box; margin-bottom: 5px;"></textarea>

                <Suspense fallback=move || view! { <span style="font-size: small;">"Loading issues..."</span> }>
                    {move || issues.get().map(|list| view! {
                        <select on:change=move |ev| set_issue_id_input.set(event_target_value(&ev)) style="width: 100%; margin-bottom: 5px;">
                            <option value="">"Select Issue (Optional)"</option>
                            <For each=move || list.clone() key=|i| i.id children=move |i| {
                                view! { <option value={i.id}>"#" {i.number} " " {i.title}</option> }
                            }/>
                        </select>
                    })}
                </Suspense>

                <button on:click=on_add_card style="width: 100%;">"Add Card"</button>
            </div>
        </div>
    }
}

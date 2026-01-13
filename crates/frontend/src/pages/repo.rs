use leptos::*;
use leptos_router::*;
use gloo_net::http::Request;
use shared::{
    Repository, CreateRepoOption, FileEntry, Issue, PullRequest, Commit, DiffFile, Branch, Tag, Release,
    Comment, CreateCommentOption, MergePullRequestOption, RepoSettingsOption, Label, CreateLabelOption,
    Milestone, CreateMilestoneOption, MilestoneStats, WikiPage, CreateWikiPageOption,
    CodeSearchResult, Collaborator, MigrateRepoOption, TransferRepoOption,
    Webhook, CreateHookOption, Secret, CreateSecretOption, DeployKey, CreateKeyOption,
    LanguageStat, ProtectedBranch, LfsLock, RepoTopicOptions, LicenseTemplate, GitignoreTemplate, UpdateFileOption,
    UpdateIssueOption, UpdatePullRequestOption, Review, CreateReviewOption
};

#[component]
pub fn RepoDetail() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let repo = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}", o, r)).send().await.unwrap().json::<Option<Repository>>().await.unwrap_or(None)
        }
    );

    let languages = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/languages", o, r))
                .send().await.unwrap().json::<Vec<LanguageStat>>().await.unwrap_or_default()
        }
    );

    let on_star = move |_| {
        let o = owner();
        let r = repo_name();
        spawn_local(async move {
            let _ = Request::post(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/star", o, r)).send().await;
        });
    };

    let on_watch = move |_| {
        let o = owner();
        let r = repo_name();
        spawn_local(async move {
            let _ = Request::post(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/watch", o, r)).send().await;
        });
    };

    let on_fork = move |_| {
        let o = owner();
        let r = repo_name();
        spawn_local(async move {
            let _ = Request::post(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/fork", o, r)).send().await;
        });
    };

    view! {
        <div class="repo-detail">
            <div class="repo-actions" style="float: right; display: flex; gap: 5px;">
                <button on:click=on_star>"Star"</button>
                <button on:click=on_watch>"Watch"</button>
                <button on:click=on_fork>"Fork"</button>
            </div>
            <h3>"Repository: " {owner} " / " {repo_name}</h3>

            <div class="repo-languages" style="margin: 10px 0;">
                <Suspense fallback=move || view! { <span></span> }>
                    {move || languages.get().map(|list| {
                        let list2 = list.clone();
                        view! {
                            <div style="display: flex; height: 10px; width: 100%; background-color: #eee; border-radius: 5px; overflow: hidden;">
                                <For each=move || list.clone() key=|l| l.language.clone() children=move |l| {
                                    view! {
                                        <div style=format!("width: {}%; background-color: {};", l.percentage, l.color) title=format!("{} {}%", l.language, l.percentage)></div>
                                    }
                                }/>
                            </div>
                            <div style="font-size: small; margin-top: 5px;">
                                <For each=move || list2.clone() key=|l| l.language.clone() children=move |l| {
                                    view! { <span style="margin-right: 10px;"><span style=format!("color: {}", l.color)>"● "</span> {l.language} " " {l.percentage} "%"</span> }
                                }/>
                            </div>
                        }
                    })}
                </Suspense>
            </div>

            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || match repo.get() {
                    Some(Some(r)) => view! {
                        <div class="repo-stats" style="margin-bottom: 10px;">
                            <span title="Stars">"⭐ " {r.stars_count}</span> " | "
                            <span title="Forks">"🍴 " {r.forks_count}</span> " | "
                            <span title="Watchers">"👁️ " {r.watchers_count}</span>
                        </div>
                        <p>"Clone URL: https://codeza.com/" {r.owner} "/" {r.name} ".git"</p>
                        <p>
                            <a href="issues">"Issues"</a> " | "
                            <a href="pulls">"Pull Requests"</a> " | "
                            <a href="src">"Code"</a> " | "
                            <a href="commits">"Commits"</a> " | "
                            <a href="releases">"Releases"</a> " | "
                            <a href="branches">"Branches"</a> " | "
                            <a href="tags">"Tags"</a> " | "
                            <a href="labels">"Labels"</a> " | "
                            <a href="milestones">"Milestones"</a> " | "
                            <a href="wiki">"Wiki"</a> " | "
                            <a href="projects">"Projects"</a> " | "
                            <a href="actions">"Actions"</a> " | "
                            <a href="settings">"Settings"</a>
                        </p>
                    }.into_view(),
                    _ => view! { <p>"Repo not found"</p> }.into_view()
                }}
            </Suspense>
        </div>
    }
}

#[component]
pub fn RepoCode() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());
    let path = move || params.with(|params| params.get("path").cloned().unwrap_or_default());

    let contents = create_resource(
        move || (owner(), repo_name(), path()),
        |(o, r, p)| async move {
            let url = if p.is_empty() {
                format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/contents", o, r)
            } else {
                format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/contents/{}", o, r, p)
            };
            Request::get(&url).send().await.unwrap().json::<Vec<FileEntry>>().await.unwrap_or_default()
        }
    );

    view! {
        <div class="repo-code">
            <h3>"Files in " {move || if path().is_empty() { "root".to_string() } else { path() }}</h3>
            <div class="code-search-link">
                <a href="search">"Search Code"</a>
            </div>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading files..."</li> }>
                    {move || contents.get().map(|files| {
                        if files.is_empty() {
                             view! { <li>"No files found or empty directory."</li> }.into_view()
                        } else {
                            view! {
                                <For each=move || files.clone() key=|f| f.path.clone() children=move |f| {
                                    let is_dir = f.kind == "dir";
                                    let link = format!("/repos/{}/{}/src/{}", owner(), repo_name(), f.path);

                                    view! {
                                        <li>
                                            {if is_dir { "📁 " } else { "📄 " }}
                                            <a href=link>{f.name}</a>
                                            " (" {f.size} " bytes)"
                                            {if !is_dir {
                                                view! { <a href=format!("/repos/{}/{}/edit/{}", owner(), repo_name(), f.path) style="margin-left: 10px;">"Edit"</a> }.into_view()
                                            } else {
                                                view! { <span></span> }.into_view()
                                            }}
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
pub fn FileEdit() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());
    let path = move || params.with(|params| params.get("path").cloned().unwrap_or_default());

    let (content, set_content) = create_signal("".to_string());
    let (message, set_message) = create_signal("Update file".to_string());

    let _ = create_resource(
        move || (owner(), repo_name(), path()),
        move |(o, r, p)| async move {
            let res = Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/raw/{}", o, r, p))
                .send().await.unwrap().text().await.unwrap_or_default();
            set_content.set(res);
        }
    );

    let on_save = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = UpdateFileOption {
            content: content.get(),
            message: message.get(),
            sha: "mock_sha".to_string(),
            branch: Some("main".to_string()),
        };
        let o = owner();
        let r = repo_name();
        let p = path();
        spawn_local(async move {
            let _ = Request::put(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/contents/{}", o, r, p))
                .json(&payload).unwrap().send().await;
        });
    };

    view! {
        <div class="file-edit">
            <h3>"Editing " {path}</h3>
            <form on:submit=on_save>
                <textarea prop:value=content on:input=move |ev| set_content.set(event_target_value(&ev)) rows="20" style="width: 100%;"></textarea>
                <input type="text" prop:value=message on:input=move |ev| set_message.set(event_target_value(&ev)) placeholder="Commit message" style="width: 100%; margin: 10px 0;" />
                <button type="submit">"Commit Changes"</button>
            </form>
        </div>
    }
}

// ... rest of the file (IssueList, IssueDetail, CreateRepo, MigrateRepo, etc.) ...
// I will include the rest of the file content from my previous read/writes to ensure no data loss.

#[component]
pub fn IssueList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (state_filter, set_state_filter) = create_signal("open".to_string());
    let (search_query, set_search_query) = create_signal("".to_string());
    let (label_filter, set_label_filter) = create_signal("".to_string());
    let (assignee_filter, set_assignee_filter) = create_signal("".to_string());
    let (sort, set_sort) = create_signal("created".to_string());
    let (direction, set_direction) = create_signal("desc".to_string());
    let (page, set_page) = create_signal(1);

    let issues = create_resource(
        move || (owner(), repo_name(), state_filter.get(), search_query.get(), label_filter.get(), assignee_filter.get(), sort.get(), direction.get(), page.get()),
        |(o, r, s, q, l, a, srt, dir, p)| async move {
            let mut url = format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/issues?state={}&q={}&sort={}&direction={}&page={}&limit=10", o, r, s, q, srt, dir, p);
            if !l.is_empty() {
                url.push_str(&format!("&label_id={}", l));
            }
            if !a.is_empty() {
                url.push_str(&format!("&assignee_username={}", a));
            }
            Request::get(&url)
                .send().await.unwrap().json::<Vec<Issue>>().await.unwrap_or_default()
        }
    );

    let labels = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/labels", o, r))
                .send().await.unwrap().json::<Vec<Label>>().await.unwrap_or_default()
        }
    );

    let users = create_resource(
        || (),
        |_| async move {
            // Mock users for filtering
            vec![
                shared::User::new(1, "admin".to_string(), None),
                shared::User::new(2, "user".to_string(), None),
            ]
        }
    );

    view! {
        <div class="issue-list">
            <h3>"Issues for " {owner} "/" {repo_name}</h3>
            <div class="issue-filters" style="display: flex; gap: 10px; align-items: center; margin-bottom: 10px;">
                <button on:click=move |_| set_state_filter.set("open".to_string()) class:active=move || state_filter.get() == "open">"Open"</button>
                <button on:click=move |_| set_state_filter.set("closed".to_string()) class:active=move || state_filter.get() == "closed">"Closed"</button>
                <button on:click=move |_| set_state_filter.set("all".to_string()) class:active=move || state_filter.get() == "all">"All"</button>

                <input type="text" placeholder="Search issues..." prop:value=search_query on:input=move |ev| set_search_query.set(event_target_value(&ev)) />

                <Suspense fallback=move || view! { <span>"Loading labels..."</span> }>
                    {move || labels.get().map(|list| view! {
                        <select on:change=move |ev| set_label_filter.set(event_target_value(&ev))>
                            <option value="">"Label"</option>
                            <For each=move || list.clone() key=|l| l.id children=move |l| {
                                view! { <option value={l.id}>{l.name}</option> }
                            }/>
                        </select>
                    })}
                </Suspense>

                <Suspense fallback=move || view! { <span>"Loading users..."</span> }>
                    {move || users.get().map(|list| view! {
                        <select on:change=move |ev| set_assignee_filter.set(event_target_value(&ev))>
                            <option value="">"Assignee"</option>
                            <For each=move || list.clone() key=|u| u.id children=move |u| {
                                view! { <option value={u.username.clone()}>{u.username}</option> }
                            }/>
                        </select>
                    })}
                </Suspense>

                <select on:change=move |ev| set_sort.set(event_target_value(&ev))>
                    <option value="created">"Created"</option>
                    <option value="updated">"Updated"</option>
                    <option value="comments">"Comments"</option>
                </select>

                <button on:click=move |_| set_direction.update(|d| *d = if d == "asc" { "desc".to_string() } else { "asc".to_string() })>
                    {move || if direction.get() == "asc" { "Asc" } else { "Desc" }}
                </button>
            </div>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading issues..."</li> }>
                    {move || issues.get().map(|list| view! {
                        <For each=move || list.clone() key=|i| i.id children=move |i| {
                            let href = format!("/repos/{}/{}/issues/{}", owner(), repo_name(), i.id);
                            view! { <li><a href=href>"#" {i.number} " " {i.title}</a> " (" {i.state} ")"</li> }
                        }/>
                    })}
                </Suspense>
            </ul>
            <div class="pagination" style="margin-top: 10px;">
                <button on:click=move |_| set_page.update(|p| if *p > 1 { *p -= 1 }) disabled=move || page.get() <= 1>"Previous"</button>
                <span style="margin: 0 10px;">"Page " {page}</span>
                <button on:click=move |_| set_page.update(|p| *p += 1)>"Next"</button>
            </div>
        </div>
    }
}

#[component]
pub fn IssueDetail() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());
    let index = move || params.with(|params| params.get("index").cloned().unwrap_or_default().parse::<u64>().unwrap_or_default());

    let (new_comment, set_new_comment) = create_signal("".to_string());
    let (trigger_refresh, set_trigger_refresh) = create_signal(0);
    let (editing_comment_id, set_editing_comment_id) = create_signal(None::<u64>);
    let (edit_comment_body, set_edit_comment_body) = create_signal("".to_string());

    let issue = create_resource(
        move || (owner(), repo_name(), index(), trigger_refresh.get()),
        |(o, r, i, _)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/issues/{}", o, r, i))
                .send().await.unwrap().json::<Option<Issue>>().await.unwrap_or(None)
        }
    );

    let comments = create_resource(
        move || (owner(), repo_name(), index(), trigger_refresh.get()),
        |(o, r, i, _)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/issues/{}/comments", o, r, i))
                .send().await.unwrap().json::<Vec<Comment>>().await.unwrap_or_default()
        }
    );

    let available_milestones = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/milestones", o, r))
                .send().await.unwrap().json::<Vec<Milestone>>().await.unwrap_or_default()
        }
    );

    let available_users = create_resource(
        || (),
        |_| async move {
            // Mock users for assignment - in real app, fetch from collaborators or org members
            vec![
                shared::User::new(1, "admin".to_string(), None),
                shared::User::new(2, "user".to_string(), None),
            ]
        }
    );

    let on_submit_comment = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = CreateCommentOption { body: new_comment.get() };
        let o = owner();
        let r = repo_name();
        let i = index();

        spawn_local(async move {
            let _ = Request::post(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/issues/{}/comments", o, r, i))
                .json(&payload).unwrap().send().await;
            set_new_comment.set("".to_string());
            set_trigger_refresh.update(|n| *n += 1);
        });
    };

    let on_delete_comment = move |comment_id: u64| {
        let o = owner();
        let r = repo_name();
        spawn_local(async move {
            let _ = Request::delete(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/issues/comments/{}", o, r, comment_id))
                .send().await;
            set_trigger_refresh.update(|n| *n += 1);
        });
    };

    let on_add_reaction = move |comment_id: u64, content: String| {
        let o = owner();
        let r = repo_name();
        let payload = shared::CreateReactionOption { content };
        spawn_local(async move {
            let _ = Request::post(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/issues/comments/{}/reactions", o, r, comment_id))
                .json(&payload).unwrap().send().await;
            set_trigger_refresh.update(|n| *n += 1);
        });
    };

    let on_start_edit_comment = move |comment_id: u64, current_body: String| {
        set_editing_comment_id.set(Some(comment_id));
        set_edit_comment_body.set(current_body);
    };

    let on_cancel_edit_comment = move |_| {
        set_editing_comment_id.set(None);
        set_edit_comment_body.set("".to_string());
    };

    let on_save_edit_comment = move |comment_id: u64| {
        let o = owner();
        let r = repo_name();
        let body = edit_comment_body.get();
        let payload = shared::UpdateCommentOption { body };

        spawn_local(async move {
            let _ = Request::patch(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/issues/comments/{}", o, r, comment_id))
                .json(&payload).unwrap().send().await;
            set_editing_comment_id.set(None);
            set_edit_comment_body.set("".to_string());
            set_trigger_refresh.update(|n| *n += 1);
        });
    };

    let on_toggle_state = move |current_state: String| {
        let o = owner();
        let r = repo_name();
        let idx = index();
        let new_state = if current_state == "open" { "closed" } else { "open" };
        let payload = UpdateIssueOption {
            title: None,
            body: None,
            state: Some(new_state.to_string()),
            milestone_id: None,
        };
        spawn_local(async move {
            let _ = Request::patch(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/issues/{}", o, r, idx))
                .json(&payload).unwrap().send().await;
            set_trigger_refresh.update(|n| *n += 1);
        });
    };

    // Label management
    let (new_label_name, set_new_label_name) = create_signal("".to_string());
    let on_add_label = move |_| {
        let o = owner();
        let r = repo_name();
        let i = index();
        let name = new_label_name.get();
        if !name.is_empty() {
            spawn_local(async move {
                let payload = CreateLabelOption { name, color: "#cccccc".to_string(), description: None };
                let _ = Request::post(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/issues/{}/labels", o, r, i))
                    .json(&payload).unwrap().send().await;
                set_new_label_name.set("".to_string());
                set_trigger_refresh.update(|n| *n += 1);
            });
        }
    };

    let on_remove_label = move |label_id: u64| {
        let o = owner();
        let r = repo_name();
        let i = index();
        spawn_local(async move {
            let _ = Request::delete(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/issues/{}/labels/{}", o, r, i, label_id))
                .send().await;
            set_trigger_refresh.update(|n| *n += 1);
        });
    };

    // Assignee management
    let (selected_assignee, set_selected_assignee) = create_signal("".to_string());
    let on_add_assignee = move |_| {
        let username = selected_assignee.get();
        if !username.is_empty() {
            let o = owner();
            let r = repo_name();
            let i = index();
            // In a real app, we'd need the full user object or ID, but the backend accepts a User struct
            // We'll construct a minimal one for the payload
            let payload = shared::User::new(0, username, None);
            spawn_local(async move {
                let _ = Request::post(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/issues/{}/assignees", o, r, i))
                    .json(&payload).unwrap().send().await;
                set_selected_assignee.set("".to_string());
                set_trigger_refresh.update(|n| *n += 1);
            });
        }
    };

    let on_remove_assignee = move |username: String| {
        let o = owner();
        let r = repo_name();
        let i = index();
        spawn_local(async move {
            let _ = Request::delete(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/issues/{}/assignees/{}", o, r, i, username))
                .send().await;
            set_trigger_refresh.update(|n| *n += 1);
        });
    };

    let on_change_milestone = move |ev: leptos::ev::Event| {
        let val_str = event_target_value(&ev);
        let m_id = val_str.parse::<u64>().unwrap_or(0);
        let o = owner();
        let r = repo_name();
        let idx = index();

        let payload = UpdateIssueOption {
            title: None,
            body: None,
            state: None,
            milestone_id: Some(m_id),
        };
        spawn_local(async move {
            let _ = Request::patch(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/issues/{}", o, r, idx))
                .json(&payload).unwrap().send().await;
            set_trigger_refresh.update(|n| *n += 1);
        });
    };

    let (is_editing, set_is_editing) = create_signal(false);
    let (edit_title, set_edit_title) = create_signal("".to_string());
    let (edit_body, set_edit_body) = create_signal("".to_string());

    let on_start_edit = move |t: String, b: String| {
        set_edit_title.set(t);
        set_edit_body.set(b);
        set_is_editing.set(true);
    };

    let on_cancel_edit = move |_| {
        set_is_editing.set(false);
    };

    let on_save_edit = move |_| {
        let o = owner();
        let r = repo_name();
        let idx = index();
        let payload = UpdateIssueOption {
            title: Some(edit_title.get()),
            body: Some(edit_body.get()),
            state: None,
            milestone_id: None,
        };
        spawn_local(async move {
            let _ = Request::patch(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/issues/{}", o, r, idx))
                .json(&payload).unwrap().send().await;
            set_is_editing.set(false);
            set_trigger_refresh.update(|n| *n += 1);
        });
    };

    view! {
        <div class="issue-detail">
            <Suspense fallback=move || view! { <p>"Loading issue..."</p> }>
                {move || match issue.get() {
                    Some(Some(i)) => {
                        let state_clone = i.state.clone();
                        let state_for_toggle = state_clone.clone();
                        let title_clone = i.title.clone();
                        let body_clone = i.body.clone().unwrap_or_default();
                        let milestone_clone = i.milestone.clone();
                        view! {
                            <div class="issue-header">
                                {if is_editing.get() {
                                    view! {
                                        <input type="text" prop:value=edit_title on:input=move |ev| set_edit_title.set(event_target_value(&ev)) style="font-size: 1.5em; width: 80%;" />
                                    }.into_view()
                                } else {
                                    view! { <h2>{i.title.clone()} " #" {i.number}</h2> }.into_view()
                                }}
                                <span class="state">{i.state.clone()}</span>
                                <span class="meta">" opened by " {i.user.username}</span>
                                <button on:click=move |_| on_toggle_state(state_for_toggle.clone()) style="margin-left: 10px;">
                                    {if state_clone == "open" { "Close Issue" } else { "Reopen Issue" }}
                                </button>
                                {if !is_editing.get() {
                                    view! { <button on:click=move |_| on_start_edit(title_clone.clone(), body_clone.clone()) style="margin-left: 5px;">"Edit"</button> }.into_view()
                                } else {
                                     view! { <span></span> }.into_view()
                                }}
                            </div>
                            <div class="issue-container" style="display: flex;">
                                <div class="issue-main" style="flex: 3;">
                                    <div class="issue-body">
                                        {if is_editing.get() {
                                            view! {
                                                <div>
                                                    <textarea prop:value=edit_body on:input=move |ev| set_edit_body.set(event_target_value(&ev)) rows="10" style="width: 100%;"></textarea>
                                                    <button on:click=on_save_edit>"Save"</button>
                                                    <button on:click=on_cancel_edit style="margin-left: 5px;">"Cancel"</button>
                                                </div>
                                            }.into_view()
                                        } else {
                                            view! { <p>{i.body.clone().unwrap_or_default()}</p> }.into_view()
                                        }}
                                    </div>
                                </div>
                                <div class="issue-sidebar" style="flex: 1; padding-left: 20px; border-left: 1px solid #eee;">
                                <div class="sidebar-item">
                                    <strong>"Assignees"</strong>
                                    <div class="assignees-list">
                                        <For each=move || i.assignees.clone() key=|u| u.id children=move |u| {
                                            let username = u.username.clone();
                                            view! {
                                                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 2px;">
                                                    <span>{username.clone()}</span>
                                                    <button on:click=move |_| on_remove_assignee(username.clone()) style="font-size: 0.8em; margin-left: 5px; cursor: pointer;">"x"</button>
                                                </div>
                                            }
                                        }/>
                                    </div>
                                    <div class="add-assignee" style="margin-top: 5px;">
                                        <Suspense fallback=move || view! { <span>"Loading users..."</span> }>
                                            {move || available_users.get().map(|users| view! {
                                                <select on:change=move |ev| set_selected_assignee.set(event_target_value(&ev))>
                                                    <option value="">"Add Assignee"</option>
                                                    <For each=move || users.clone() key=|u| u.id children=move |u| {
                                                        let username = u.username.clone();
                                                        view! { <option value={username.clone()}>{username}</option> }
                                                    }/>
                                                </select>
                                                <button on:click=on_add_assignee>"+"</button>
                                            })}
                                        </Suspense>
                                    </div>
                                </div>
                                <div class="sidebar-item">
                                    <strong>"Labels"</strong>
                                    <div class="labels-list">
                                        <For each=move || i.labels.clone() key=|l| l.id children=move |l| {
                                            let label_id = l.id;
                                            view! {
                                                <div style=format!("background-color: {}; color: #fff; padding: 2px 5px; border-radius: 3px; display: inline-block; margin-right: 5px; margin-bottom: 2px;", l.color)>
                                                    {l.name}
                                                    <span on:click=move |_| on_remove_label(label_id) style="margin-left: 5px; cursor: pointer; font-weight: bold;">"x"</span>
                                                </div>
                                            }
                                        }/>
                                    </div>
                                    <div class="add-label" style="margin-top: 5px;">
                                        <input type="text" prop:value=new_label_name on:input=move |ev| set_new_label_name.set(event_target_value(&ev)) placeholder="New Label" style="width: 100px;"/>
                                        <button on:click=on_add_label>"+"</button>
                                    </div>
                                </div>
                                <div class="sidebar-item">
                                    <strong>"Milestone"</strong>
                                    <div>
                                        <Suspense fallback=move || view! { <span>"Loading..."</span> }>
                                            {
                                                let milestone_clone_2 = milestone_clone.clone();
                                                move || available_milestones.get().map(|list| {
                                                    let current_id = milestone_clone_2.as_ref().map(|m| m.id).unwrap_or(0);
                                                    view! {
                                                        <select on:change=on_change_milestone>
                                                            <option value="0" selected={current_id == 0}>"No Milestone"</option>
                                                            <For each=move || list.clone() key=|m| m.id children=move |m| {
                                                                let selected = m.id == current_id;
                                                                view! { <option value={m.id} selected={selected}>{m.title}</option> }
                                                            }/>
                                                        </select>
                                                    }
                                                })
                                            }
                                        </Suspense>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }.into_view() },
                    _ => view! { <p>"Issue not found"</p> }.into_view()
                }}
            </Suspense>

            <div class="comments-section">
                <h3>"Comments"</h3>
                <Suspense fallback=move || view! { <p>"Loading comments..."</p> }>
                    {move || comments.get().map(|list| view! {
                        <For each=move || list.clone() key=|c| c.id children=move |c| {
                            let comment_id = c.id;
                            let comment_body = c.body.clone();
                            view! {
                                <div class="comment">
                                    <div class="comment-header">
                                        <strong>{c.user.username}</strong> " commented on " {c.created_at}
                                        {
                                            // Mock admin check: allow edit/delete for everyone in mock
                                            view! {
                                                <span style="float: right;">
                                                    <button on:click=move |_| on_start_edit_comment(comment_id, comment_body.clone())>"Edit"</button>
                                                    <button on:click=move |_| on_delete_comment(comment_id)>"Delete"</button>
                                                </span>
                                            }
                                        }
                                    </div>
                                    <div class="comment-body">
                                        {move || if editing_comment_id.get() == Some(comment_id) {
                                            view! {
                                                <div>
                                                    <textarea prop:value=edit_comment_body on:input=move |ev| set_edit_comment_body.set(event_target_value(&ev))></textarea>
                                                    <button on:click=move |_| on_save_edit_comment(comment_id)>"Save"</button>
                                                    <button on:click=on_cancel_edit_comment>"Cancel"</button>
                                                </div>
                                            }.into_view()
                                        } else {
                                            view! { <p>{c.body.clone()}</p> }.into_view()
                                        }}
                                    </div>
                                    <div class="comment-reactions" style="margin-top: 5px;">
                                        <div class="reactions-list" style="display: flex; gap: 5px; margin-bottom: 5px;">
                                            <For each=move || c.reactions.clone() key=|r| r.id children=move |r| {
                                                view! { <span title={r.user.username} style="border: 1px solid #ddd; padding: 2px 5px; border-radius: 10px;">{r.content}</span> }
                                            }/>
                                        </div>
                                        <div class="reaction-picker">
                                            <button on:click=move |_| on_add_reaction(comment_id, "👍".to_string()) title="+1">"👍"</button>
                                            <button on:click=move |_| on_add_reaction(comment_id, "👎".to_string()) title="-1">"👎"</button>
                                            <button on:click=move |_| on_add_reaction(comment_id, "😄".to_string()) title="laugh">"😄"</button>
                                            <button on:click=move |_| on_add_reaction(comment_id, "😕".to_string()) title="confused">"😕"</button>
                                            <button on:click=move |_| on_add_reaction(comment_id, "❤️".to_string()) title="heart">"❤️"</button>
                                            <button on:click=move |_| on_add_reaction(comment_id, "🎉".to_string()) title="hooray">"🎉"</button>
                                            <button on:click=move |_| on_add_reaction(comment_id, "👀".to_string()) title="eyes">"👀"</button>
                                            <button on:click=move |_| on_add_reaction(comment_id, "🚀".to_string()) title="rocket">"🚀"</button>
                                        </div>
                                    </div>
                                </div>
                            }
                        }/>
                    })}
                </Suspense>

                <form on:submit=on_submit_comment class="comment-form">
                    <textarea
                        prop:value=new_comment
                        on:input=move |ev| set_new_comment.set(event_target_value(&ev))
                        placeholder="Leave a comment"
                    ></textarea>
                    <button type="submit">"Comment"</button>
                </form>
            </div>
        </div>
    }
}

#[component]
pub fn CreateRepo() -> impl IntoView {
    let (name, set_name) = create_signal("".to_string());
    let (desc, set_desc) = create_signal("".to_string());
    let (gitignore, set_gitignore) = create_signal("".to_string());
    let (license, set_license) = create_signal("".to_string());

    let licenses = create_resource(|| (), |_| async move {
        Request::get("http://127.0.0.1:3000/api/v1/licenses").send().await.unwrap().json::<Vec<LicenseTemplate>>().await.unwrap_or_default()
    });

    let gitignores = create_resource(|| (), |_| async move {
        Request::get("http://127.0.0.1:3000/api/v1/gitignore/templates").send().await.unwrap().json::<Vec<GitignoreTemplate>>().await.unwrap_or_default()
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = CreateRepoOption {
            name: name.get(),
            description: if desc.get().is_empty() { None } else { Some(desc.get()) },
            private: false,
            auto_init: true,
            gitignores: if gitignore.get().is_empty() { None } else { Some(gitignore.get()) },
            license: if license.get().is_empty() { None } else { Some(license.get()) },
            readme: Some("Default".to_string()),
        };
        spawn_local(async move {
            let _ = Request::post("http://127.0.0.1:3000/api/v1/user/repos").json(&payload).unwrap().send().await;
        });
    };

    view! {
        <div class="create-repo">
            <h3>"Create New Repository"</h3>
            <form on:submit=on_submit>
                <input type="text" placeholder="Repository Name" prop:value=name on:input=move |ev| set_name.set(event_target_value(&ev)) />
                <input type="text" placeholder="Description" prop:value=desc on:input=move |ev| set_desc.set(event_target_value(&ev)) />

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


#[component]
pub fn CommitList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let commits = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/commits", o, r))
                .send().await.unwrap().json::<Vec<Commit>>().await.unwrap_or_default()
        }
    );

    view! {
        <div class="commit-list">
            <h3>"Commit History for " {owner} "/" {repo_name}</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading commits..."</li> }>
                    {move || commits.get().map(|list| view! {
                        <For each=move || list.clone() key=|c| c.sha.clone() children=move |c| {
                            let href = format!("/repos/{}/{}/commits/{}", owner(), repo_name(), c.sha);
                            view! {
                                <li>
                                    <a href=href class="commit-sha">{c.sha.chars().take(7).collect::<String>()}</a>
                                    " - "
                                    <span class="commit-message">{c.message}</span>
                                    " by "
                                    <span class="commit-author">{c.author.username}</span>
                                    " on "
                                    <span class="commit-date">{c.date}</span>
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
pub fn CommitDiff() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());
    let sha = move || params.with(|params| params.get("sha").cloned().unwrap_or_default());

    let diffs = create_resource(
        move || (owner(), repo_name(), sha()),
        |(o, r, s)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/commits/{}/diff", o, r, s))
                .send().await.unwrap().json::<Vec<DiffFile>>().await.unwrap_or_default()
        }
    );

    view! {
        <div class="commit-diff">
            <h3>"Commit Diff: " {sha}</h3>
            <div class="diff-container">
                <Suspense fallback=move || view! { <p>"Loading diff..."</p> }>
                    {move || diffs.get().map(|files| view! {
                        <For each=move || files.clone() key=|f| f.name.clone() children=move |f| {
                            view! {
                                <div class="file-diff">
                                    <div class="file-header">
                                        <strong>{f.name}</strong>
                                        <span class="diff-stats">
                                            " +"{f.additions} " -"{f.deletions}
                                        </span>
                                    </div>
                                    <pre class="diff-content">
                                        <For each=move || f.lines.clone() key=|l| format!("{}{:?}", l.content, l.line_no_old) children=move |line| {
                                            let class_name = match line.type_.as_str() {
                                                "add" => "diff-line-add",
                                                "delete" => "diff-line-delete",
                                                _ => "diff-line-context",
                                            };
                                            view! { <div class=class_name>{line.content}</div> }
                                        }/>
                                    </pre>
                                </div>
                            }
                        }/>
                    })}
                </Suspense>
            </div>

        </div>
    }
}

#[component]
pub fn PullRequestList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let pulls = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/pulls", o, r))
                .send().await.unwrap().json::<Vec<PullRequest>>().await.unwrap_or_default()
        }
    );

    view! {
        <div class="pull-list">
            <h3>"Pull Requests for " {owner} "/" {repo_name}</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading pull requests..."</li> }>
                    {move || pulls.get().map(|list| view! {
                        <For each=move || list.clone() key=|p| p.id children=move |p| {
                            let href = format!("/repos/{}/{}/pulls/{}", owner(), repo_name(), p.id);
                            view! { <li><a href=href>"#" {p.number} " " {p.title}</a> " (" {p.state} ")"</li> }
                        }/>
                    })}
                </Suspense>
            </ul>
        </div>
    }
}

#[component]
pub fn PullRequestDetail() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());
    let index = move || params.with(|params| params.get("index").cloned().unwrap_or_default().parse::<u64>().unwrap_or_default());

    let (merge_action, set_merge_action) = create_signal("merge".to_string());
    let (trigger_refresh, set_trigger_refresh) = create_signal(0);

    // Fetch PR details to display status, title, body etc.
    let pull_request = create_resource(
        move || (owner(), repo_name(), index(), trigger_refresh.get()),
        |(o, r, i, _)| async move {
            // Note: list_pulls filters by repo, but we need get_pull. Since get_pull logic is inside list_pulls basically,
            // we might not have a direct endpoint for get_pull in router yet? No, router has `list_pulls` but no `get_pull`.
            // Wait, looking at router.rs: `.route("/api/v1/repos/:owner/:repo/pulls", get(list_pulls)...)`
            // There isn't a `get_pull` route! We should add one or iterate list (inefficient but works for now).
            // Actually, we can use the `list_pulls` and find the one with the right index client-side or add endpoint.
            // For now, let's filter client side from list since that endpoint exists.
            let pulls = Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/pulls", o, r))
                .send().await.unwrap().json::<Vec<PullRequest>>().await.unwrap_or_default();
            pulls.into_iter().find(|p| p.number == i)
        }
    );

    let pr_files = create_resource(
        move || (owner(), repo_name(), index()),
        |(o, r, i)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/pulls/{}/files", o, r, i))
                .send().await.unwrap().json::<Vec<DiffFile>>().await.unwrap_or_default()
        }
    );

    let on_merge = move |_| {
        let o = owner();
        let r = repo_name();
        let i = index();
        let action = merge_action.get();
        spawn_local(async move {
            let payload = MergePullRequestOption {
                merge_action: action,
                merge_title_field: None,
                merge_message_field: None,
            };
            let _ = Request::post(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/pulls/{}/merge", o, r, i))
                .json(&payload).unwrap().send().await;
            set_trigger_refresh.update(|n| *n += 1);
        });
    };

    let on_toggle_state = move |current_state: String| {
        let o = owner();
        let r = repo_name();
        let idx = index();
        let new_state = if current_state == "open" { "closed" } else { "open" };
        let payload = UpdatePullRequestOption {
            title: None,
            body: None,
            state: Some(new_state.to_string()),
        };
        spawn_local(async move {
            let _ = Request::patch(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/pulls/{}", o, r, idx))
                .json(&payload).unwrap().send().await;
            set_trigger_refresh.update(|n| *n += 1);
        });
    };

    let (is_editing, set_is_editing) = create_signal(false);
    let (edit_title, set_edit_title) = create_signal("".to_string());
    let (edit_body, set_edit_body) = create_signal("".to_string());

    let on_start_edit = move |t: String, b: String| {
        set_edit_title.set(t);
        set_edit_body.set(b);
        set_is_editing.set(true);
    };

    let on_cancel_edit = move |_| {
        set_is_editing.set(false);
    };

    let on_save_edit = move |_| {
        let o = owner();
        let r = repo_name();
        let idx = index();
        let payload = UpdatePullRequestOption {
            title: Some(edit_title.get()),
            body: Some(edit_body.get()),
            state: None,
        };
        spawn_local(async move {
            let _ = Request::patch(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/pulls/{}", o, r, idx))
                .json(&payload).unwrap().send().await;
            set_is_editing.set(false);
            set_trigger_refresh.update(|n| *n += 1);
        });
    };

    let reviews = create_resource(
        move || (owner(), repo_name(), index(), trigger_refresh.get()),
        |(o, r, i, _)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/pulls/{}/reviews", o, r, i))
                .send().await.unwrap().json::<Vec<Review>>().await.unwrap_or_default()
        }
    );

    let (review_body, set_review_body) = create_signal("".to_string());

    let on_submit_review = move |event: String| {
        let o = owner();
        let r = repo_name();
        let i = index();
        let payload = CreateReviewOption {
            body: review_body.get(),
            event,
        };
        spawn_local(async move {
            let _ = Request::post(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/pulls/{}/reviews", o, r, i))
                .json(&payload).unwrap().send().await;
            set_review_body.set("".to_string());
            set_trigger_refresh.update(|n| *n += 1);
        });
    };

    view! {
        <div class="pull-detail">
            <Suspense fallback=move || view! { <p>"Loading PR..."</p> }>
                {move || match pull_request.get() {
                    Some(Some(pr)) => {
                        let state_clone = pr.state.clone();
                        let state_for_toggle = state_clone.clone();
                        let title_clone = pr.title.clone();
                        let body_clone = pr.body.clone().unwrap_or_default();

                        view! {
                            <div class="pr-header">
                                {if is_editing.get() {
                                    view! {
                                        <input type="text" prop:value=edit_title on:input=move |ev| set_edit_title.set(event_target_value(&ev)) style="font-size: 1.5em; width: 80%;" />
                                    }.into_view()
                                } else {
                                    view! { <h3>"Pull Request #" {index} ": " {pr.title.clone()}</h3> }.into_view()
                                }}
                                <span class="state">{pr.state.clone()}</span>
                                <span class="meta">" opened by " {pr.user.username}</span>
                                <button on:click=move |_| on_toggle_state(state_for_toggle.clone()) style="margin-left: 10px;">
                                    {if state_clone == "open" { "Close PR" } else { "Reopen PR" }}
                                </button>
                                {if !is_editing.get() {
                                    view! { <button on:click=move |_| on_start_edit(title_clone.clone(), body_clone.clone()) style="margin-left: 5px;">"Edit"</button> }.into_view()
                                } else {
                                     view! { <span></span> }.into_view()
                                }}
                            </div>
                            <div class="pr-body" style="margin: 10px 0; padding: 10px; border: 1px solid #eee;">
                                {if is_editing.get() {
                                    view! {
                                        <div>
                                            <textarea prop:value=edit_body on:input=move |ev| set_edit_body.set(event_target_value(&ev)) rows="5" style="width: 100%;"></textarea>
                                            <button on:click=on_save_edit>"Save"</button>
                                            <button on:click=on_cancel_edit style="margin-left: 5px;">"Cancel"</button>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <p>{pr.body.clone().unwrap_or_default()}</p> }.into_view()
                                }}
                            </div>
                        }
                    }.into_view(),
                    _ => view! { <p>"Pull Request not found"</p> }.into_view()
                }}
            </Suspense>

            <div class="pr-actions">
                <select on:change=move |ev| set_merge_action.set(event_target_value(&ev))>
                    <option value="merge">"Merge Commit"</option>
                    <option value="rebase">"Rebase and Merge"</option>
                    <option value="squash">"Squash and Merge"</option>
                </select>
                <button on:click=on_merge class="btn-merge">"Merge Pull Request"</button>
            </div>
            <div class="pr-files">
                <h4>"Files Changed"</h4>
                <Suspense fallback=move || view! { <p>"Loading files..."</p> }>
                    {move || pr_files.get().map(|files| view! {
                        <For each=move || files.clone() key=|f| f.name.clone() children=move |f| {
                             view! {
                                <div class="file-diff">
                                    <div class="file-header">
                                        <strong>{f.name}</strong>
                                        <span class="diff-stats">" +"{f.additions} " -"{f.deletions}</span>
                                    </div>
                                    <p>"Binary or large file diff suppressed"</p>
                                </div>
                            }
                        }/>
                    })}
                </Suspense>
            </div>

            <div class="pr-reviews" style="margin-top: 20px; border-top: 1px solid #eee; padding-top: 10px;">
                <h4>"Reviews"</h4>
                <div class="review-list">
                    <Suspense fallback=move || view! { <p>"Loading reviews..."</p> }>
                        {move || reviews.get().map(|list| view! {
                            <For each=move || list.clone() key=|r| r.id children=move |r| {
                                view! {
                                    <div class="review-item" style="border: 1px solid #ddd; padding: 10px; margin-bottom: 10px; border-radius: 5px;">
                                        <div class="review-header">
                                            <strong>{r.user.username}</strong> " "
                                            <span style=format!("font-weight: bold; color: {}", match r.state.as_str() {
                                                "APPROVED" => "green",
                                                "CHANGES_REQUESTED" => "red",
                                                _ => "gray"
                                            })>{r.state}</span>
                                            " on " {r.created_at}
                                        </div>
                                        <div class="review-body" style="margin-top: 5px;">
                                            {r.body}
                                        </div>
                                    </div>
                                }
                            }/>
                        })}
                    </Suspense>
                </div>

                <div class="add-review" style="margin-top: 10px; border: 1px solid #ccc; padding: 10px;">
                    <h5>"Submit Review"</h5>
                    <textarea prop:value=review_body on:input=move |ev| set_review_body.set(event_target_value(&ev)) placeholder="Leave a comment" style="width: 100%; margin-bottom: 5px;"></textarea>
                    <div style="display: flex; gap: 5px;">
                        <button on:click=move |_| on_submit_review("COMMENT".to_string())>"Comment"</button>
                        <button on:click=move |_| on_submit_review("APPROVE".to_string()) style="color: green;">"Approve"</button>
                        <button on:click=move |_| on_submit_review("REQUEST_CHANGES".to_string()) style="color: red;">"Request Changes"</button>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn BranchList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let branches = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/branches", o, r))
                .send().await.unwrap().json::<Vec<Branch>>().await.unwrap_or_default()
        }
    );

    view! {
        <div class="branch-list">
            <h3>"Branches for " {owner} "/" {repo_name}</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading branches..."</li> }>
                    {move || branches.get().map(|list| view! {
                        <For each=move || list.clone() key=|b| b.name.clone() children=move |b| {
                            view! {
                                <li>
                                    <strong>{b.name}</strong>
                                    {if b.protected { " (Protected)" } else { "" }}
                                    " - " {b.commit.sha.chars().take(7).collect::<String>()}
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
pub fn TagList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let tags = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/tags", o, r))
                .send().await.unwrap().json::<Vec<Tag>>().await.unwrap_or_default()
        }
    );

    view! {
        <div class="tag-list">
            <h3>"Tags for " {owner} "/" {repo_name}</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading tags..."</li> }>
                    {move || tags.get().map(|list| view! {
                        <For each=move || list.clone() key=|t| t.name.clone() children=move |t| {
                            view! {
                                <li>
                                    <strong>{t.name}</strong>
                                    " - " {t.commit.sha.chars().take(7).collect::<String>()}
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
pub fn ReleaseList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let releases = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/releases", o, r))
                .send().await.unwrap().json::<Vec<Release>>().await.unwrap_or_default()
        }
    );

    view! {
        <div class="release-list">
            <h3>"Releases for " {owner} "/" {repo_name}</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading releases..."</li> }>
                    {move || releases.get().map(|list| view! {
                        <For each=move || list.clone() key=|r| r.id children=move |r| {
                            view! {
                                <li>
                                    <strong>{r.name}</strong> " (" {r.tag_name} ")"
                                    {if r.draft { " [Draft]" } else { "" }}
                                    {if r.prerelease { " [Pre-release]" } else { "" }}
                                    <p>{r.body.unwrap_or_default()}</p>
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
pub fn RepoSettings() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (desc, set_desc) = create_signal("".to_string());
    let (transfer_to, set_transfer_to) = create_signal("".to_string());
    let (topics, set_topics) = create_signal("".to_string());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = RepoSettingsOption {
            description: Some(desc.get()),
            private: None,
            website: None,
        };
        let o = owner();
        let r = repo_name();
        spawn_local(async move {
            let _ = Request::patch(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/settings", o, r))
                .json(&payload).unwrap().send().await;
        });
    };

    let on_transfer = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = TransferRepoOption { new_owner: transfer_to.get() };
        let o = owner();
        let r = repo_name();
        spawn_local(async move {
            let _ = Request::post(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/transfer", o, r))
                .json(&payload).unwrap().send().await;
        });
    };

    let on_update_topics = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let topic_list: Vec<String> = topics.get().split(',').map(|s| s.trim().to_string()).collect();
        let payload = RepoTopicOptions { topics: topic_list };
        let o = owner();
        let r = repo_name();
        spawn_local(async move {
            let _ = Request::put(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/topics", o, r))
                .json(&payload).unwrap().send().await;
        });
    };

    let on_sync = move |_| {
        let o = owner();
        let r = repo_name();
        spawn_local(async move {
            let _ = Request::post(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/mirror-sync", o, r)).send().await;
        });
    };

    view! {
        <div class="repo-settings">
            <h3>"Repository Settings"</h3>
            <form on:submit=on_submit>
                <label>"Description"</label>
                <input type="text" prop:value=desc on:input=move |ev| set_desc.set(event_target_value(&ev)) />
                <button type="submit">"Update Settings"</button>
            </form>

            <h4>"Topics"</h4>
            <form on:submit=on_update_topics>
                <input type="text" placeholder="rust, gitea, clone" prop:value=topics on:input=move |ev| set_topics.set(event_target_value(&ev)) />
                <button type="submit">"Update Topics"</button>
            </form>

            <h4>"Mirror Settings"</h4>
            <button on:click=on_sync>"Sync Now"</button>

            <h4>"Transfer Ownership"</h4>
            <form on:submit=on_transfer>
                <input type="text" placeholder="New Owner Username" prop:value=transfer_to on:input=move |ev| set_transfer_to.set(event_target_value(&ev)) />
                <button type="submit">"Transfer"</button>
            </form>

            <div class="settings-sections">
                <p><a href="collaborators">"Collaborators"</a></p>
                <p><a href="webhooks">"Webhooks"</a></p>
                <p><a href="secrets">"Secrets"</a></p>
                <p><a href="keys">"Deploy Keys"</a></p>
                <p><a href="branches">"Protected Branches"</a></p>
                <p><a href="lfs">"Git LFS Locks"</a></p>
            </div>
        </div>
    }
}

#[component]
pub fn ProtectedBranchList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let branches = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/branch_protections", o, r))
                .send().await.unwrap().json::<Vec<ProtectedBranch>>().await.unwrap_or_default()
        }
    );

    view! {
        <div class="protected-branches">
            <h3>"Protected Branches"</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading..."</li> }>
                    {move || branches.get().map(|list| view! {
                        <For each=move || list.clone() key=|b| b.name.clone() children=move |b| {
                            view! { <li>{b.name} " (Push: " {b.enable_push} ", Force: " {b.enable_force_push} ")"</li> }
                        }/>
                    })}
                </Suspense>
            </ul>
        </div>
    }
}

#[component]
pub fn LfsLockList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let locks = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/git/lfs/locks", o, r))
                .send().await.unwrap().json::<Vec<LfsLock>>().await.unwrap_or_default()
        }
    );

    view! {
        <div class="lfs-locks">
            <h3>"Git LFS Locks"</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading..."</li> }>
                    {move || locks.get().map(|list| view! {
                        <For each=move || list.clone() key=|l| l.id.clone() children=move |l| {
                            view! { <li>{l.path} " locked by " {l.owner.username}</li> }
                        }/>
                    })}
                </Suspense>
            </ul>
        </div>
    }
}

#[component]
pub fn WebhookList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (url, set_url) = create_signal("".to_string());

    let hooks = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/hooks", o, r))
                .send().await.unwrap().json::<Vec<Webhook>>().await.unwrap_or_default()
        }
    );

    let on_create = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = CreateHookOption {
            url: url.get(),
            events: vec!["push".to_string()],
            active: true,
        };
        let o = owner();
        let r = repo_name();
        spawn_local(async move {
             let _ = Request::post(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/hooks", o, r))
                .json(&payload).unwrap().send().await;
            set_url.set("".to_string());
        });
    };

    view! {
        <div class="webhook-list">
            <h3>"Webhooks"</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading..."</li> }>
                    {move || hooks.get().map(|list| view! {
                        <For each=move || list.clone() key=|h| h.id children=move |h| {
                            view! { <li>{h.url} " (" {if h.active { "Active" } else { "Inactive" }} ")"</li> }
                        }/>
                    })}
                </Suspense>
            </ul>
            <h4>"Add Webhook"</h4>
            <form on:submit=on_create>
                <input type="text" placeholder="Payload URL" prop:value=url on:input=move |ev| set_url.set(event_target_value(&ev)) />
                <button type="submit">"Add Webhook"</button>
            </form>
        </div>
    }
}

#[component]
pub fn SecretList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (name, set_name) = create_signal("".to_string());
    let (data, set_data) = create_signal("".to_string());

    let secrets = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/secrets", o, r))
                .send().await.unwrap().json::<Vec<Secret>>().await.unwrap_or_default()
        }
    );

    let on_create = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = CreateSecretOption {
            name: name.get(),
            data: data.get(),
        };
        let o = owner();
        let r = repo_name();
        spawn_local(async move {
             let _ = Request::post(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/secrets", o, r))
                .json(&payload).unwrap().send().await;
            set_name.set("".to_string());
            set_data.set("".to_string());
        });
    };

    view! {
        <div class="secret-list">
            <h3>"Secrets"</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading..."</li> }>
                    {move || secrets.get().map(|list| view! {
                        <For each=move || list.clone() key=|s| s.name.clone() children=move |s| {
                            view! { <li>{s.name}</li> }
                        }/>
                    })}
                </Suspense>
            </ul>
            <h4>"Add Secret"</h4>
            <form on:submit=on_create>
                <input type="text" placeholder="Name" prop:value=name on:input=move |ev| set_name.set(event_target_value(&ev)) />
                <input type="text" placeholder="Value" prop:value=data on:input=move |ev| set_data.set(event_target_value(&ev)) />
                <button type="submit">"Add Secret"</button>
            </form>
        </div>
    }
}

#[component]
pub fn DeployKeyList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (title, set_title) = create_signal("".to_string());
    let (key, set_key) = create_signal("".to_string());

    let keys = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/keys", o, r))
                .send().await.unwrap().json::<Vec<DeployKey>>().await.unwrap_or_default()
        }
    );

    let on_create = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = CreateKeyOption {
            title: title.get(),
            key: key.get(),
        };
        let o = owner();
        let r = repo_name();
        spawn_local(async move {
             let _ = Request::post(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/keys", o, r))
                .json(&payload).unwrap().send().await;
            set_title.set("".to_string());
            set_key.set("".to_string());
        });
    };

    view! {
        <div class="deploy-key-list">
            <h3>"Deploy Keys"</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading..."</li> }>
                    {move || keys.get().map(|list| view! {
                        <For each=move || list.clone() key=|k| k.id children=move |k| {
                            view! { <li>{k.title} " - " {k.fingerprint}</li> }
                        }/>
                    })}
                </Suspense>
            </ul>
            <h4>"Add Deploy Key"</h4>
            <form on:submit=on_create>
                <input type="text" placeholder="Title" prop:value=title on:input=move |ev| set_title.set(event_target_value(&ev)) />
                <textarea placeholder="Key" prop:value=key on:input=move |ev| set_key.set(event_target_value(&ev))></textarea>
                <button type="submit">"Add Key"</button>
            </form>
        </div>
    }
}

#[component]
pub fn CollaboratorList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());
    let (new_collab, set_new_collab) = create_signal("".to_string());

    let collabs = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/collaborators", o, r))
                .send().await.unwrap().json::<Vec<Collaborator>>().await.unwrap_or_default()
        }
    );

    let on_add = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let o = owner();
        let r = repo_name();
        let c = new_collab.get();
        spawn_local(async move {
            let _ = Request::put(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/collaborators/{}", o, r, c))
                .send().await;
            set_new_collab.set("".to_string());
        });
    };

    view! {
        <div class="collaborators">
            <h3>"Collaborators"</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading..."</li> }>
                    {move || collabs.get().map(|list| view! {
                        <For each=move || list.clone() key=|c| c.user.id children=move |c| {
                            view! { <li>{c.user.username} " (" {c.permissions} ")"</li> }
                        }/>
                    })}
                </Suspense>
            </ul>
            <form on:submit=on_add>
                <input type="text" placeholder="Username" prop:value=new_collab on:input=move |ev| set_new_collab.set(event_target_value(&ev)) />
                <button type="submit">"Add Collaborator"</button>
            </form>
        </div>
    }
}

#[component]
pub fn LabelList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (name, set_name) = create_signal("".to_string());
    let (color, set_color) = create_signal("#000000".to_string());

    let labels = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/labels", o, r))
                .send().await.unwrap().json::<Vec<Label>>().await.unwrap_or_default()
        }
    );

    let on_create = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = CreateLabelOption {
            name: name.get(),
            color: color.get(),
            description: None,
        };
        let o = owner();
        let r = repo_name();
        spawn_local(async move {
             let _ = Request::post(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/labels", o, r))
                .json(&payload).unwrap().send().await;
            set_name.set("".to_string());
        });
    };

    view! {
        <div class="label-list">
            <h3>"Labels"</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading..."</li> }>
                    {move || labels.get().map(|list| view! {
                        <For each=move || list.clone() key=|l| l.id children=move |l| {
                            view! {
                                <li style=format!("border-left: 5px solid {}", l.color)>
                                    {l.name}
                                </li>
                            }
                        }/>
                    })}
                </Suspense>
            </ul>
            <h4>"Create Label"</h4>
            <form on:submit=on_create>
                <input type="text" placeholder="Name" prop:value=name on:input=move |ev| set_name.set(event_target_value(&ev)) />
                <input type="color" prop:value=color on:input=move |ev| set_color.set(event_target_value(&ev)) />
                <button type="submit">"Create"</button>
            </form>
        </div>
    }
}

#[component]
pub fn MilestoneList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (title, set_title) = create_signal("".to_string());

    let milestones = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/milestones", o, r))
                .send().await.unwrap().json::<Vec<Milestone>>().await.unwrap_or_default()
        }
    );

    let on_create = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = CreateMilestoneOption {
            title: title.get(),
            description: None,
            due_on: None,
        };
        let o = owner();
        let r = repo_name();
        spawn_local(async move {
             let _ = Request::post(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/milestones", o, r))
                .json(&payload).unwrap().send().await;
            set_title.set("".to_string());
        });
    };

    view! {
        <div class="milestone-list">
            <h3>"Milestones"</h3>
             <ul>
                <Suspense fallback=move || view! { <li>"Loading..."</li> }>
                    {move || milestones.get().map(|list| view! {
                        <For each=move || list.clone() key=|m| m.id children=move |m| {
                             let href = format!("/repos/{}/{}/milestones/{}", owner(), repo_name(), m.id);
                            view! {
                                <li>
                                    <a href=href>{m.title}</a> " (" {m.state} ")"
                                </li>
                            }
                        }/>
                    })}
                </Suspense>
            </ul>
             <h4>"Create Milestone"</h4>
            <form on:submit=on_create>
                <input type="text" placeholder="Title" prop:value=title on:input=move |ev| set_title.set(event_target_value(&ev)) />
                <button type="submit">"Create"</button>
            </form>
        </div>
    }
}

#[component]
pub fn MilestoneDetail() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());
    let index = move || params.with(|params| params.get("index").cloned().unwrap_or_default().parse::<u64>().unwrap_or_default());

    let milestone = create_resource(
         move || (owner(), repo_name(), index()),
        |(o, r, i)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/milestones/{}", o, r, i))
                .send().await.unwrap().json::<Option<Milestone>>().await.unwrap_or(None)
        }
    );

    let stats = create_resource(
         move || (owner(), repo_name(), index()),
        |(o, r, i)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/milestones/{}/stats", o, r, i))
                .send().await.unwrap().json::<MilestoneStats>().await.unwrap_or(MilestoneStats { open_issues: 0, closed_issues: 0 })
        }
    );

    view! {
        <div class="milestone-detail">
             <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || match milestone.get() {
                    Some(Some(m)) => view! {
                        <h3>"Milestone: " {m.title}</h3>
                        <p>{m.description.unwrap_or_default()}</p>
                        <p>"State: " {m.state}</p>
                    }.into_view(),
                    _ => view! { <p>"Milestone not found"</p> }.into_view()
                }}
            </Suspense>
             <Suspense fallback=move || view! { <p>"Loading stats..."</p> }>
                {move || stats.get().map(|s| view! {
                    <div class="stats">
                        <span>"Open Issues: " {s.open_issues}</span>
                        " | "
                        <span>"Closed Issues: " {s.closed_issues}</span>
                    </div>
                })}
            </Suspense>
        </div>
    }
}

#[component]
pub fn Wiki() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());
    let page_name = move || params.with(|params| params.get("page_name").cloned().unwrap_or("Home".to_string()));

    let wiki_page = create_resource(
        move || (owner(), repo_name(), page_name()),
        move |(o, r, p)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/wiki/pages/{}", o, r, p))
                .send().await.unwrap().json::<Option<WikiPage>>().await.unwrap_or(None)
        }
    );

    view! {
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
    }
}

#[component]
pub fn WikiEdit() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());
    let page_name = move || params.with(|params| params.get("page_name").cloned().unwrap_or("Home".to_string()));

    let (content, set_content) = create_signal("".to_string());
    let (message, set_message) = create_signal("".to_string());

    // Load existing content if available
    let _ = create_resource(
        move || (owner(), repo_name(), page_name()),
        move |(o, r, p)| async move {
             if let Ok(resp) = Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/wiki/pages/{}", o, r, p)).send().await {
                 if let Ok(Some(page)) = resp.json::<Option<WikiPage>>().await {
                     set_content.set(page.content);
                 }
             }
        }
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
        spawn_local(async move {
            let _ = Request::post(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/wiki/pages", o, r))
                .json(&payload).unwrap().send().await;
            // Redirect or notify would happen here
        });
    };

    view! {
        <div class="wiki-edit">
            <h3>"Editing " {page_name}</h3>
            <form on:submit=on_submit>
                <textarea prop:value=content on:input=move |ev| set_content.set(event_target_value(&ev)) rows="10"></textarea>
                <input type="text" placeholder="Commit Message" prop:value=message on:input=move |ev| set_message.set(event_target_value(&ev)) />
                <button type="submit">"Save Page"</button>
            </form>
        </div>
    }
}



#[component]
pub fn RepoCodeSearch() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());
    let (query, set_query) = create_signal("".to_string());

    // Trigger resource when query changes (and is not empty)
    let search_results = create_resource(
        move || (owner(), repo_name(), query.get()),
        |(o, r, q)| async move {
            if q.is_empty() { return vec![]; }
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/search?q={}", o, r, q))
                .send().await.unwrap().json::<Vec<CodeSearchResult>>().await.unwrap_or_default()
        }
    );

    let on_search = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        // search_results resource will auto-update because it depends on `query()`
    };

    view! {
        <div class="repo-search">
            <h3>"Search Code"</h3>
            <form on:submit=on_search>
                <input type="text" prop:value=query on:input=move |ev| set_query.set(event_target_value(&ev)) placeholder="Search..."/>
                <button type="submit">"Search"</button>
            </form>
            <ul>
                <Suspense fallback=move || view! { <li>"Searching..."</li> }>
                    {move || search_results.get().map(|list| view! {
                        <For each=move || list.clone() key=|r| r.path.clone() children=move |r| {
                            let path_clone = r.path.clone();
                            view! {
                                <li>
                                    <strong>{r.path}</strong>
                                    // Link to line number or file
                                    <a href=format!("src/{}", path_clone)>"View"</a>
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
pub fn MigrateRepo() -> impl IntoView {
    let (clone_addr, set_clone_addr) = create_signal("".to_string());
    let (repo_name, set_repo_name) = create_signal("".to_string());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = MigrateRepoOption {
            clone_addr: clone_addr.get(),
            repo_name: repo_name.get(),
            service: "git".to_string(),
            mirror: false,
        };
        spawn_local(async move {
            let _ = Request::post("http://127.0.0.1:3000/api/v1/repos/migrate").json(&payload).unwrap().send().await;
        });
    };

    view! {
        <div class="migrate-repo">
            <h3>"Migrate Repository"</h3>
            <form on:submit=on_submit>
                <input type="text" placeholder="Clone URL" prop:value=clone_addr on:input=move |ev| set_clone_addr.set(event_target_value(&ev)) />
                <input type="text" placeholder="Repository Name" prop:value=repo_name on:input=move |ev| set_repo_name.set(event_target_value(&ev)) />
                <button type="submit">"Migrate"</button>
            </form>
        </div>
    }
}

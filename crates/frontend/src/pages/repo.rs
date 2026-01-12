use leptos::*;
use leptos_router::*;
use gloo_net::http::Request;
use shared::{
    Repository, CreateRepoOption, Package, FileEntry, Issue, PullRequest, Commit, DiffFile, Branch, Tag, Release,
    Comment, CreateCommentOption, MergePullRequestOption, RepoSettingsOption, Label, CreateLabelOption,
    Milestone, CreateMilestoneOption, MilestoneStats, WikiPage, CreateWikiPageOption, Project,
    ActionWorkflow, CodeSearchResult, Collaborator, MigrateRepoOption, TransferRepoOption,
    Webhook, CreateHookOption, Secret, CreateSecretOption, DeployKey, CreateKeyOption,
    LanguageStat, ProtectedBranch, LfsLock, RepoTopicOptions, LicenseTemplate, GitignoreTemplate, UpdateFileOption
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

    let issues = create_resource(
        move || (owner(), repo_name(), state_filter.get(), search_query.get()),
        |(o, r, s, q)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/issues?state={}&q={}", o, r, s, q))
                .send().await.unwrap().json::<Vec<Issue>>().await.unwrap_or_default()
        }
    );

    view! {
        <div class="issue-list">
            <h3>"Issues for " {owner} "/" {repo_name}</h3>
            <div class="issue-filters">
                <button on:click=move |_| set_state_filter.set("open".to_string()) class:active=move || state_filter.get() == "open">"Open"</button>
                <button on:click=move |_| set_state_filter.set("closed".to_string()) class:active=move || state_filter.get() == "closed">"Closed"</button>
                <button on:click=move |_| set_state_filter.set("all".to_string()) class:active=move || state_filter.get() == "all">"All"</button>
                <input type="text" placeholder="Search issues..." prop:value=search_query on:input=move |ev| set_search_query.set(event_target_value(&ev)) />
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

    let issue = create_resource(
        move || (owner(), repo_name(), index()),
        |(o, r, i)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/issues/{}", o, r, i))
                .send().await.unwrap().json::<Option<Issue>>().await.unwrap_or(None)
        }
    );

    let comments = create_resource(
        move || (owner(), repo_name(), index()),
        |(o, r, i)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/issues/{}/comments", o, r, i))
                .send().await.unwrap().json::<Vec<Comment>>().await.unwrap_or_default()
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
        });
    };

    view! {
        <div class="issue-detail">
            <Suspense fallback=move || view! { <p>"Loading issue..."</p> }>
                {move || match issue.get() {
                    Some(Some(i)) => view! {
                        <div class="issue-header">
                            <h2>{i.title} " #" {i.number}</h2>
                            <span class="state">{i.state}</span>
                            <span class="meta">" opened by " {i.user.username}</span>
                        </div>
                        <div class="issue-container" style="display: flex;">
                            <div class="issue-main" style="flex: 3;">
                                <div class="issue-body">
                                    <p>{i.body.unwrap_or_default()}</p>
                                </div>
                            </div>
                            <div class="issue-sidebar" style="flex: 1; padding-left: 20px; border-left: 1px solid #eee;">
                                <div class="sidebar-item">
                                    <strong>"Assignees"</strong>
                                    <div class="assignees-list">
                                        <For each=move || i.assignees.clone() key=|u| u.id children=move |u| {
                                            view! { <div>{u.username}</div> }
                                        }/>
                                    </div>
                                </div>
                                <div class="sidebar-item">
                                    <strong>"Labels"</strong>
                                    <div class="labels-list">
                                        <For each=move || i.labels.clone() key=|l| l.id children=move |l| {
                                            view! { <div style=format!("background-color: {}; color: #fff; padding: 2px 5px; border-radius: 3px; display: inline-block; margin-right: 5px;", l.color)>{l.name}</div> }
                                        }/>
                                    </div>
                                </div>
                                <div class="sidebar-item">
                                    <strong>"Milestone"</strong>
                                    <p>{i.milestone.clone().map(|m| m.title).unwrap_or("No milestone".to_string())}</p>
                                </div>
                            </div>
                        </div>
                    }.into_view(),
                    _ => view! { <p>"Issue not found"</p> }.into_view()
                }}
            </Suspense>

            <div class="comments-section">
                <h3>"Comments"</h3>
                <Suspense fallback=move || view! { <p>"Loading comments..."</p> }>
                    {move || comments.get().map(|list| view! {
                        <For each=move || list.clone() key=|c| c.id children=move |c| {
                            view! {
                                <div class="comment">
                                    <div class="comment-header">
                                        <strong>{c.user.username}</strong> " commented on " {c.created_at}
                                    </div>
                                    <div class="comment-body">{c.body}</div>
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
pub fn PackageList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());

    let packages = create_resource(owner, |owner_name| async move {
        Request::get(&format!("http://127.0.0.1:3000/api/v1/packages/{}", owner_name)).send().await.unwrap().json::<Vec<Package>>().await.unwrap_or_default()
    });

    view! {
        <div class="package-list">
            <h3>"Packages for " {owner}</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading..."</li> }>
                    {move || packages.get().map(|list| view! {
                        <For each=move || list.clone() key=|p| p.id children=move |p| {
                            let href = format!("/packages/{}/{}/{}/{}", owner(), p.package_type, p.name, p.version);
                            view! { <li><a href=href>{p.name} " (" {p.package_type} ") - " {p.version}</a></li> }
                        }/>
                    })}
                </Suspense>
            </ul>
        </div>
    }
}

#[component]
pub fn PackageDetail() -> impl IntoView {
    let params = use_params_map();
    let name = move || params.with(|params| params.get("name").cloned().unwrap_or_default());
    let version = move || params.with(|params| params.get("version").cloned().unwrap_or_default());

    view! {
        <div class="package-detail">
            <h3>"Package: " {name} " " {version}</h3>
            <p>"Installation instructions..."</p>
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
        });
    };

    view! {
        <div class="pull-detail">
            <h3>"Pull Request #" {index}</h3>
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
                    _ => view! {
                        <div>
                            <p>"Wiki page '" {page_name} "' not found."</p>
                            <a href=format!("/repos/{}/{}/wiki/pages/{}/edit", owner(), repo_name(), page_name())>
                                "Create " {page_name} " Page"
                            </a>
                        </div>
                    }.into_view()
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
pub fn ProjectList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let projects = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/projects", o, r))
                .send().await.unwrap().json::<Vec<Project>>().await.unwrap_or_default()
        }
    );

    view! {
        <div class="project-list">
            <h3>"Projects"</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading projects..."</li> }>
                    {move || projects.get().map(|list| view! {
                        <For each=move || list.clone() key=|p| p.id children=move |p| {
                            view! {
                                <li>
                                    <strong>{p.title}</strong>
                                    <p>{p.description.unwrap_or_default()}</p>
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
pub fn ActionsList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let actions = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/actions/workflows", o, r))
                .send().await.unwrap().json::<Vec<ActionWorkflow>>().await.unwrap_or_default()
        }
    );

    view! {
        <div class="actions-list">
            <h3>"Actions Workflows"</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading workflows..."</li> }>
                    {move || actions.get().map(|list| view! {
                        <For each=move || list.clone() key=|w| w.id children=move |w| {
                            view! {
                                <li>
                                    <strong>{w.name}</strong> " - " {w.status}
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

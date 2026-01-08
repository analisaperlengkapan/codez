use leptos::*;
use leptos_router::*;
use gloo_net::http::Request;
use shared::{
    Repository, CreateRepoOption, Package, FileEntry, Issue, PullRequest, Commit, DiffFile, Branch, Tag, Release,
    Comment, CreateCommentOption, MergePullRequestOption
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

    view! {
        <div class="repo-detail">
            <h3>"Repository: " {owner} " / " {repo_name}</h3>
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || match repo.get() {
                    Some(Some(r)) => view! {
                        <p>"Clone URL: https://codeza.com/" {r.owner} "/" {r.name} ".git"</p>
                        <p>
                            <a href="issues">"Issues"</a> " | "
                            <a href="pulls">"Pull Requests"</a> " | "
                            <a href="src">"Code"</a> " | "
                            <a href="commits">"Commits"</a> " | "
                            <a href="releases">"Releases"</a> " | "
                            <a href="branches">"Branches"</a> " | "
                            <a href="tags">"Tags"</a>
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
    // "path" param handles the *path wildcard in routing
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
pub fn IssueList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let issues = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/issues", o, r))
                .send().await.unwrap().json::<Vec<Issue>>().await.unwrap_or_default()
        }
    );

    view! {
        <div class="issue-list">
            <h3>"Issues for " {owner} "/" {repo_name}</h3>
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
            // In a real app we would refetch comments here
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
                        <div class="issue-body">
                            <p>{i.body.unwrap_or_default()}</p>
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

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = CreateRepoOption {
            name: name.get(),
            description: None,
            private: false,
            auto_init: true,
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
                <button type="submit">"Create"</button>
            </form>
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
        spawn_local(async move {
            let payload = MergePullRequestOption {
                merge_action: "merge".to_string(),
                merge_title_field: None,
                merge_message_field: None,
            };
            let _ = Request::post(&format!("http://127.0.0.1:3000/api/v1/repos/{}/{}/pulls/{}/merge", o, r, i))
                .json(&payload).unwrap().send().await;
            // Refresh logic would go here
        });
    };

    view! {
        <div class="pull-detail">
            <h3>"Pull Request #" {index}</h3>
            <div class="pr-actions">
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
                                    // Reuse DiffLine logic if extracted, or simplified view here
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

// Stub components to allow compilation
#[component]
pub fn RepoCodeSearch() -> impl IntoView { view! { <div>"Repo Search Placeholder"</div> } }
#[component]
pub fn ActionsList() -> impl IntoView { view! { <div>"Actions List Placeholder"</div> } }
#[component]
pub fn LabelList() -> impl IntoView { view! { <div>"Label List Placeholder"</div> } }
#[component]
pub fn MilestoneList() -> impl IntoView { view! { <div>"Milestone List Placeholder"</div> } }
#[component]
pub fn MilestoneDetail() -> impl IntoView { view! { <div>"Milestone Detail Placeholder"</div> } }
#[component]
pub fn ProjectList() -> impl IntoView { view! { <div>"Project List Placeholder"</div> } }
#[component]
pub fn Wiki() -> impl IntoView { view! { <div>"Wiki Placeholder"</div> } }
#[component]
pub fn WikiEdit() -> impl IntoView { view! { <div>"Wiki Edit Placeholder"</div> } }
#[component]
pub fn RepoSettings() -> impl IntoView { view! { <div>"Repo Settings Placeholder"</div> } }

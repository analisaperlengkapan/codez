use leptos::*;
use leptos_router::*;
use shared::{ActionWorkflow, Activity, AdminStats, AdminUserEditOption, Branch, CodeSearchResult, Collaborator, Comment, Commit, Contribution, CreateBranchOption, CreateCommentOption, CreateGpgKeyOption, CreateHookOption, CreateIssueOption, CreateKeyOption, CreateLabelOption, CreateMilestoneOption, CreatePullRequestOption, CreateReactionOption, CreateReleaseOption, CreateRepoOption, CreateSecretOption, CreateWikiPageOption, DeployKey, DiffFile, DiffLine, EmailAddress, FileEntry, GitignoreTemplate, GpgKey, Issue, LfsObject, Label, LanguageStat, LicenseTemplate, LoginOption, MergePullRequestOption, MigrateRepoOption, Milestone, MilestoneStats, Notification, OAuth2Application, OAuth2Provider, Organization, OrgMember, Package, Project, ProtectedBranch, PublicKey, PullRequest, Reaction, RegisterOption, Release, RepoActionOption, RepoSettingsOption, RepoTopicOptions, Repository, ReviewRequest, Secret, SystemNotice, Tag, Team, Topic, TransferRepoOption, TwoFactor, User, UserSettingsOption, Webhook, WikiPage};

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <a href="/">"Home"</a> " | "
                <a href="/explore">"Explore"</a> " | "
                <a href="/search">"Search"</a> " | "
                <a href="/notifications">"Notifications"</a> " | "
                <a href="/admin">"Admin"</a> " | "
                <a href="/login">"Login"</a> " | "
                <a href="/register">"Register"</a> " | "
                <a href="/users/admin">"Admin Profile"</a> " | "
                <a href="/orgs/codeza-org">"Org Profile"</a> " | "
                <a href="/repo/create">"New Repo"</a>
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=UserDashboard/>
                    <Route path="/explore" view=Explore/>
                    <Route path="/admin" view=AdminDashboard/>
                    <Route path="/admin/users" view=AdminUsers/>
                    <Route path="/search" view=Search/>
                    <Route path="/repos/:owner/:repo/search" view=RepoCodeSearch/>
                    <Route path="/packages/:owner" view=PackageList/>
                    <Route path="/packages/:owner/:type/:name/:version" view=PackageDetail/>
                    <Route path="/notifications" view=NotificationList/>
                    <Route path="/login" view=Login/>
                    <Route path="/register" view=Register/>
                    <Route path="/repo/create" view=CreateRepo/>
                    <Route path="/users/:username" view=UserProfile/>
                    <Route path="/users/:username/followers" view=UserFollowers/>
                    <Route path="/users/:username/following" view=UserFollowing/>
                    <Route path="/settings/profile" view=UserSettings/>
                    <Route path="/orgs/:org" view=OrgProfile/>
                    <Route path="/repos/:owner/:repo" view=RepoDetail/>
                    <Route path="/repos/:owner/:repo/issues" view=IssueList/>
                    <Route path="/repos/:owner/:repo/issues/:index" view=IssueDetail/>
                    <Route path="/repos/:owner/:repo/pulls" view=PullRequestList/>
                    <Route path="/repos/:owner/:repo/pulls/:index" view=PullRequestDetail/>
                    <Route path="/repos/:owner/:repo/actions" view=ActionsList/>
                    <Route path="/repos/:owner/:repo/branches" view=BranchList/>
                    <Route path="/repos/:owner/:repo/tags" view=TagList/>
                    <Route path="/repos/:owner/:repo/src/*path" view=RepoCode/>
                    <Route path="/repos/:owner/:repo/commits" view=CommitList/>
                    <Route path="/repos/:owner/:repo/commits/:sha/diff" view=CommitDiff/>
                    <Route path="/repos/:owner/:repo/releases" view=ReleaseList/>
                    <Route path="/repos/:owner/:repo/labels" view=LabelList/>
                    <Route path="/repos/:owner/:repo/milestones" view=MilestoneList/>
                    <Route path="/repos/:owner/:repo/milestones/:index" view=MilestoneDetail/>
                    <Route path="/repos/:owner/:repo/projects" view=ProjectList/>
                    <Route path="/repos/:owner/:repo/wiki" view=Wiki/>
                    <Route path="/repos/:owner/:repo/wiki/pages/:page_name/edit" view=WikiEdit/>
                    <Route path="/repos/:owner/:repo/settings" view=RepoSettings/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! { <UserDashboard/> }
}

#[component]
fn UserDashboard() -> impl IntoView {
    let (repos, set_repos) = create_signal(vec![]);

    create_effect(move |_| {
        let mock_repos = vec![
            Repository::new(1, "codeza".to_string(), "admin".to_string()),
            Repository::new(2, "gitea-clone".to_string(), "user".to_string()),
        ];
        set_repos.set(mock_repos);
    });

    view! {
        <div class="dashboard container">
            <div class="dashboard-sidebar">
                <h3>"My Repositories"</h3>
                <ul>
                    <For each=move || repos.get() key=|repo| repo.id children=move |repo| {
                        view! { <li><a href=format!("/repos/{}/{}", repo.owner, repo.name)>{repo.name}</a></li> }
                    }/>
                </ul>
                <h3>"Organizations"</h3>
                <p><a href="/orgs/codeza-org">"codeza-org"</a></p>
            </div>
            <div class="dashboard-content">
                <ActivityFeed/>
            </div>
        </div>
    }
}

#[component]
fn ActivityFeed() -> impl IntoView {
    let (activities, set_activities) = create_signal(vec![]);
    create_effect(move |_| {
        // Mock
        let mock = vec![
            Activity {
                id: 1,
                user_id: 1,
                user_name: "admin".to_string(),
                op_type: "push_branch".to_string(),
                content: "pushed to main".to_string(),
                created: "just now".to_string(),
            }
        ];
        set_activities.set(mock);
    });

    view! {
        <div class="activity-feed">
            <h2>"Activity Feed"</h2>
            <ul>
                <For
                    each=move || activities.get()
                    key=|a| a.id
                    children=move |a| {
                        view! {
                            <li>
                                <strong>{a.user_name}</strong> " " {a.op_type} ": " {a.content}
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn AdminDashboard() -> impl IntoView {
    let (stats, set_stats) = create_signal(None::<AdminStats>);
    create_effect(move |_| {
        // Mock
        set_stats.set(Some(AdminStats { users: 10, repos: 20, orgs: 5, issues: 100 }));
    });

    view! {
        <div class="admin-dashboard">
            <h2>"Admin Dashboard"</h2>
            <p><a href="/admin/users">"Manage Users"</a></p>
            {move || match stats.get() {
                Some(s) => view! {
                    <div>
                        <p>"Users: " {s.users}</p>
                        <p>"Repos: " {s.repos}</p>
                        <p>"Orgs: " {s.orgs}</p>
                        <p>"Issues: " {s.issues}</p>
                    </div>
                }.into_view(),
                None => view! { <p>"Loading..."</p> }.into_view()
            }}
            <AdminNotices/>
        </div>
    }
}

#[component]
fn AdminUsers() -> impl IntoView {
    let (users, set_users) = create_signal(vec![]);
    create_effect(move |_| {
        set_users.set(vec![
            User::new(1, "admin".to_string(), Some("admin@example.com".to_string())),
            User::new(2, "user".to_string(), Some("user@example.com".to_string())),
        ]);
    });

    view! {
        <div class="admin-users">
            <h3>"User Management"</h3>
            <table>
                <thead><tr><th>"ID"</th><th>"Username"</th><th>"Email"</th><th>"Actions"</th></tr></thead>
                <tbody>
                    <For each=move || users.get() key=|u| u.id children=move |u| {
                        view! {
                            <tr>
                                <td>{u.id}</td>
                                <td>{u.username}</td>
                                <td>{u.email}</td>
                                <td>
                                    <button>"Edit"</button>
                                    <button>"Delete"</button>
                                </td>
                            </tr>
                        }
                    }/>
                </tbody>
            </table>
        </div>
    }
}

#[component]
fn AdminNotices() -> impl IntoView {
    let (notices, set_notices) = create_signal(vec![]);
    create_effect(move |_| {
        set_notices.set(vec![
            SystemNotice { id: 1, type_: "info".to_string(), description: "System update".to_string() }
        ]);
    });

    view! {
        <div class="admin-notices">
            <h3>"System Notices"</h3>
            <ul>
                <For
                    each=move || notices.get()
                    key=|n| n.id
                    children=move |n| {
                        view! {
                            <li>[{n.type_}] {n.description}</li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn NotificationList() -> impl IntoView {
    let (notifs, set_notifs) = create_signal(vec![]);
    create_effect(move |_| {
        let mock = vec![
            Notification { id: 1, subject: "Welcome".to_string(), unread: true, updated_at: "now".to_string() }
        ];
        set_notifs.set(mock);
    });

    view! {
        <div class="notifications">
            <h2>"Notifications"</h2>
            <ul>
                <For
                    each=move || notifs.get()
                    key=|n| n.id
                    children=move |n| {
                        view! {
                            <li>
                                <strong>{n.subject}</strong> " (" {if n.unread { "Unread" } else { "Read" }} ")"
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn Search() -> impl IntoView {
    let (query, set_query) = create_signal("".to_string());
    let (results, set_results) = create_signal(vec![]);

    let on_search = move |_| {
        // Mock search
        let q = query.get();
        if !q.is_empty() {
            let mock = vec![
                Repository::new(1, format!("{}-repo", q), "user".to_string())
            ];
            set_results.set(mock);
        }
    };

    view! {
        <div class="search-page">
            <h2>"Search Repositories"</h2>
            <input type="text" placeholder="Search..."
                prop:value=query
                on:input=move |ev| set_query.set(event_target_value(&ev))
            />
            <button on:click=on_search>"Search"</button>
            <ul>
                <For
                    each=move || results.get()
                    key=|r| r.id
                    children=move |r| {
                        view! {
                            <li>{r.owner} "/" {r.name}</li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn Explore() -> impl IntoView {
    view! {
        <div class="explore">
            <h2>"Explore Codeza"</h2>
            <Search/>
        </div>
    }
}

#[component]
fn PackageList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());

    let (packages, set_packages) = create_signal(vec![]);
    create_effect(move |_| {
         // Mock
         let mock = vec![
             Package { id: 1, name: "lib-rs".to_string(), version: "0.1.0".to_string(), package_type: "cargo".to_string() }
         ];
         set_packages.set(mock);
    });

    view! {
        <div class="package-list">
            <h3>"Packages for " {owner}</h3>
            <ul>
                <For
                    each=move || packages.get()
                    key=|p| p.id
                    children=move |p| {
                         let href = format!("/packages/{}/{}/{}/{}", owner(), p.package_type, p.name, p.version);
                        view! {
                            <li><a href=href>{p.name} " (" {p.package_type} ") - " {p.version}</a></li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn PackageDetail() -> impl IntoView {
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
fn Login() -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = LoginOption {
            username: username.get(),
            password: password.get(),
        };
        leptos::logging::log!("Logging in: {:?}", payload);
    };

    view! {
        <div class="login">
            <h2>"Login"</h2>
            <form on:submit=on_submit>
                <input type="text" placeholder="Username" prop:value=username
                    on:input=move |ev| set_username.set(event_target_value(&ev)) />
                <input type="password" placeholder="Password" prop:value=password
                    on:input=move |ev| set_password.set(event_target_value(&ev)) />
                <button type="submit">"Login"</button>
            </form>
            <div class="oauth2">
                <h3>"Or sign in with:"</h3>
                <ul>
                    <li><a href="/auth/github">"GitHub"</a></li>
                </ul>
            </div>
        </div>
    }
}

#[component]
fn Register() -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());
    let (email, set_email) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = RegisterOption {
            username: username.get(),
            email: email.get(),
            password: password.get(),
        };
        leptos::logging::log!("Registering: {:?}", payload);
    };

    view! {
        <div class="register">
            <h2>"Register"</h2>
            <form on:submit=on_submit>
                <input type="text" placeholder="Username" prop:value=username
                    on:input=move |ev| set_username.set(event_target_value(&ev)) />
                <input type="email" placeholder="Email" prop:value=email
                    on:input=move |ev| set_email.set(event_target_value(&ev)) />
                <input type="password" placeholder="Password" prop:value=password
                    on:input=move |ev| set_password.set(event_target_value(&ev)) />
                <button type="submit">"Register"</button>
            </form>
        </div>
    }
}

#[component]
fn OrgProfile() -> impl IntoView {
    let params = use_params_map();
    let org_name = move || params.with(|params| params.get("org").cloned().unwrap_or_default());

    let org = create_memo(move |_| {
         if org_name() == "codeza-org" {
             Some(Organization {
                 id: 1,
                 username: "codeza-org".to_string(),
                 description: Some("Description".to_string()),
                 avatar_url: None,
             })
         } else {
             None
         }
    });

    view! {
        <div class="org-profile">
            <h2>"Organization: " {org_name}</h2>
            {move || match org.get() {
                Some(o) => view! {
                    <p>{o.description.unwrap_or_default()}</p>
                }.into_view(),
                None => view! { <p>"Org not found"</p> }.into_view()
            }}
            <TeamList/>
            <OrgMembers/>
        </div>
    }
}

#[component]
fn OrgMembers() -> impl IntoView {
    let params = use_params_map();
    let org = move || params.with(|params| params.get("org").cloned().unwrap_or_default());

    let (members, set_members) = create_signal(vec![]);
    create_effect(move |_| {
         // Mock
         let mock = vec![
             OrgMember { user: User::new(1, "admin".to_string(), None), role: "owner".to_string() }
         ];
         set_members.set(mock);
    });

    view! {
        <div class="org-members">
            <h3>"Members of " {org}</h3>
            <ul>
                <For each=move || members.get() key=|m| m.user.id children=move |m| {
                    view! { <li>{m.user.username} " (" {m.role} ")"</li> }
                }/>
            </ul>
        </div>
    }
}

#[component]
fn UserProfile() -> impl IntoView {
    let params = use_params_map();
    let username = move || params.with(|params| params.get("username").cloned().unwrap_or_default());

    // Mock user fetch
    let user = create_memo(move |_| {
         if username() == "admin" {
             Some(User::new(1, "admin".to_string(), Some("admin@codeza.com".to_string())))
         } else {
             None
         }
    });

    view! {
        <div class="user-profile">
            <h2>"User Profile: " {username}</h2>
            {move || match user.get() {
                Some(u) => view! {
                    <div>
                        <p>"Email: " {u.email.unwrap_or("Hidden".to_string())}</p>
                        <p>
                            <a href=format!("/users/{}/followers", u.username)>"Followers"</a> " | "
                            <a href=format!("/users/{}/following", u.username)>"Following"</a>
                        </p>
                        <UserHeatmap/>
                    </div>
                }.into_view(),
                None => view! { <p>"User not found"</p> }.into_view()
            }}
        </div>
    }
}

#[component]
fn UserHeatmap() -> impl IntoView {
    let params = use_params_map();
    let username = move || params.with(|params| params.get("username").cloned().unwrap_or_default());
    let (data, set_data) = create_signal(vec![]);

    create_effect(move |_| {
        set_data.set(vec![
            Contribution { date: "2023-01-01".to_string(), count: 5 }
        ]);
    });

    view! {
        <div class="user-heatmap">
            <h3>"Contributions for " {username}</h3>
            <div class="calendar-stub">
                <For each=move || data.get() key=|c| c.date.clone() children=move |c| {
                     view! {
                         <div title=format!("{} commits on {}", c.count, c.date) style="display:inline-block; width: 10px; height: 10px; background-color: green; margin: 1px;"></div>
                     }
                }/>
            </div>
        </div>
    }
}

#[component]
fn UserFollowers() -> impl IntoView {
    let params = use_params_map();
    let username = move || params.with(|params| params.get("username").cloned().unwrap_or_default());
    let (users, set_users) = create_signal(vec![]);

    create_effect(move |_| {
        set_users.set(vec![User::new(2, "follower".to_string(), None)]);
    });

    view! {
        <div class="followers">
            <h3>"Followers of " {username}</h3>
            <ul>
                <For each=move || users.get() key=|u| u.id children=move |u| {
                    view! { <li>{u.username}</li> }
                }/>
            </ul>
        </div>
    }
}

#[component]
fn UserFollowing() -> impl IntoView {
    let params = use_params_map();
    let username = move || params.with(|params| params.get("username").cloned().unwrap_or_default());
    let (users, set_users) = create_signal(vec![]);

    create_effect(move |_| {
        set_users.set(vec![User::new(3, "following".to_string(), None)]);
    });

    view! {
        <div class="following">
            <h3>{username} " is following"</h3>
            <ul>
                <For each=move || users.get() key=|u| u.id children=move |u| {
                    view! { <li>{u.username}</li> }
                }/>
            </ul>
        </div>
    }
}

#[component]
fn CreateRepo() -> impl IntoView {
    let (name, set_name) = create_signal("".to_string());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = CreateRepoOption {
            name: name.get(),
            description: None,
            private: false,
            auto_init: true,
        };
        // In a real app, we would POST this payload
        leptos::logging::log!("Creating repo: {:?}", payload);
    };

    view! {
        <div class="create-repo">
            <h3>"Create New Repository"</h3>
            <p><a href="/repo/migrate">"Import Repository"</a></p>
            <form on:submit=on_submit>
                <input
                    type="text"
                    placeholder="Repository Name"
                    prop:value=name
                    on:input=move |ev| set_name.set(event_target_value(&ev))
                />
                <div class="template-selectors">
                    <select>
                        <option value="">"Select .gitignore Template"</option>
                        <option value="Rust">"Rust"</option>
                    </select>
                    <select>
                        <option value="">"Select License"</option>
                        <option value="MIT">"MIT"</option>
                    </select>
                </div>
                <button type="submit">"Create"</button>
            </form>
        </div>
    }
}

#[component]
fn MigrateRepo() -> impl IntoView {
    let (url, set_url) = create_signal("".to_string());
    let (name, set_name) = create_signal("".to_string());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = MigrateRepoOption {
            clone_addr: url.get(),
            repo_name: name.get(),
            service: "git".to_string(),
            mirror: false,
        };
        leptos::logging::log!("Migrating repo: {:?}", payload);
    };

    view! {
        <div class="migrate-repo">
            <h3>"Migrate / Import Repository"</h3>
            <form on:submit=on_submit>
                <input type="text" placeholder="Clone URL" prop:value=url on:input=move |ev| set_url.set(event_target_value(&ev)) />
                <input type="text" placeholder="Repository Name" prop:value=name on:input=move |ev| set_name.set(event_target_value(&ev)) />
                <button type="submit">"Migrate Repo"</button>
            </form>
        </div>
    }
}

#[component]
fn RepoDetail() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let issues_href = move || format!("/repos/{}/{}/issues", owner(), repo_name());
    let pulls_href = move || format!("/repos/{}/{}/pulls", owner(), repo_name());
    let code_href = move || format!("/repos/{}/{}/src/", owner(), repo_name());
    let commits_href = move || format!("/repos/{}/{}/commits", owner(), repo_name());
    let releases_href = move || format!("/repos/{}/{}/releases", owner(), repo_name());
    let labels_href = move || format!("/repos/{}/{}/labels", owner(), repo_name());
    let milestones_href = move || format!("/repos/{}/{}/milestones", owner(), repo_name());
    let projects_href = move || format!("/repos/{}/{}/projects", owner(), repo_name());
    let wiki_href = move || format!("/repos/{}/{}/wiki", owner(), repo_name());
    let settings_href = move || format!("/repos/{}/{}/settings", owner(), repo_name());
    let actions_href = move || format!("/repos/{}/{}/actions", owner(), repo_name());
    let branches_href = move || format!("/repos/{}/{}/branches", owner(), repo_name());
    let tags_href = move || format!("/repos/{}/{}/tags", owner(), repo_name());

    view! {
        <div class="repo-detail">
            <h3>"Repository: " {owner} " / " {repo_name}</h3>
            <div class="repo-actions">
                <button on:click=move |_| leptos::logging::log!("Watch")>"Watch"</button>
                <button on:click=move |_| leptos::logging::log!("Star")>"Star"</button>
                <button on:click=move |_| leptos::logging::log!("Fork")>"Fork"</button>
            </div>
            <p>"Clone URL: https://codeza.com/" {owner} "/" {repo_name} ".git"</p>
            <p>
                <a href=code_href>"Code"</a> " | "
                <a href=commits_href>"Commits"</a> " | "
                <a href=branches_href>"Branches"</a> " | "
                <a href=tags_href>"Tags"</a> " | "
                <a href=issues_href>"Issues"</a> " | "
                <a href=pulls_href>"Pull Requests"</a> " | "
                <a href=releases_href>"Releases"</a> " | "
                <a href=labels_href>"Labels"</a> " | "
                <a href=milestones_href>"Milestones"</a> " | "
                <a href=projects_href>"Projects"</a> " | "
                <a href=wiki_href>"Wiki"</a> " | "
                <a href=actions_href>"Actions"</a> " | "
                <a href=settings_href>"Settings"</a>
            </p>
            <TopicList/>
            <RepoLanguages/>
        </div>
    }
}

#[component]
fn RepoLanguages() -> impl IntoView {
    let params = use_params_map();
    let (stats, set_stats) = create_signal(vec![]);
    create_effect(move |_| {
        set_stats.set(vec![
            LanguageStat { language: "Rust".to_string(), percentage: 100, color: "#dea584".to_string() }
        ]);
    });

    view! {
        <div class="repo-languages">
            <div class="lang-bar" style="display: flex; height: 10px; border-radius: 5px; overflow: hidden;">
                <For each=move || stats.get() key=|s| s.language.clone() children=move |s| {
                    view! {
                        <div style=format!("width: {}%; background-color: {}", s.percentage, s.color) title=format!("{} {}%", s.language, s.percentage)></div>
                    }
                }/>
            </div>
            <ul>
                <For each=move || stats.get() key=|s| s.language.clone() children=move |s| {
                    view! {
                        <li><span style=format!("color: {}", s.color)>"●"</span> {s.language} " " {s.percentage} "%"</li>
                    }
                }/>
            </ul>
        </div>
    }
}

#[component]
fn ActionsList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (workflows, set_workflows) = create_signal(vec![]);
    create_effect(move |_| {
         // Mock
         let mock = vec![
             ActionWorkflow { id: 1, name: "Build".to_string(), status: "Success".to_string() }
         ];
         set_workflows.set(mock);
    });

    view! {
        <div class="actions-list">
            <h3>"Actions for " {owner} " / " {repo_name}</h3>
            <ul>
                <For
                    each=move || workflows.get()
                    key=|w| w.id
                    children=move |w| {
                        view! {
                            <li>{w.name} " - " {w.status}</li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn TopicList() -> impl IntoView {
    let params = use_params_map();

    let (topics, set_topics) = create_signal(vec![]);
    create_effect(move |_| {
         let mock = vec![
             Topic { id: 1, name: "rust".to_string(), created: "2023-01-01".to_string() }
         ];
         set_topics.set(mock);
    });

    view! {
        <div class="topic-list">
            <ul>
                <For
                    each=move || topics.get()
                    key=|t| t.id
                    children=move |t| {
                        view! {
                            <li>
                                <span class="topic">{t.name}</span>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn ReleaseList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (releases, set_releases) = create_signal(vec![]);

    create_effect(move |_| {
        // Mock contents
        let user = User::new(1, "admin".to_string(), None);
        let mock = vec![
            Release {
                id: 1,
                tag_name: "v1.0.0".to_string(),
                name: "Initial".to_string(),
                body: Some("Desc".to_string()),
                draft: false,
                prerelease: false,
                created_at: "2023-01-01".to_string(),
                author: user,
            }
        ];
        set_releases.set(mock);
    });

    view! {
        <div class="release-list">
            <h3>"Releases for " {owner} " / " {repo_name}</h3>
            <ul>
                <For
                    each=move || releases.get()
                    key=|r| r.id
                    children=move |r| {
                        view! {
                            <li>
                                <strong>{r.tag_name}</strong> " - " {r.name}
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn CommitList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (commits, set_commits) = create_signal(vec![]);

    create_effect(move |_| {
        // Mock contents
        let user = User::new(1, "admin".to_string(), None);
        let mock = vec![
            Commit {
                sha: "1234567".to_string(),
                message: "Init".to_string(),
                author: user,
                date: "2023-01-01".to_string(),
            }
        ];
        set_commits.set(mock);
    });

    view! {
        <div class="commit-list">
            <h3>"Commits for " {owner} " / " {repo_name}</h3>
            <ul>
                <For
                    each=move || commits.get()
                    key=|c| c.sha.clone()
                    children=move |c| {
                        let diff_href = format!("/repos/{}/{}/commits/{}/diff", owner(), repo_name(), c.sha);
                        view! {
                            <li>
                                <a href=diff_href><code>{c.sha}</code></a> " - " {c.message} " (" {c.author.username} ")"
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn CommitDiff() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());
    let sha = move || params.with(|params| params.get("sha").cloned().unwrap_or_default());

    let (diffs, set_diffs) = create_signal(vec![]);

    create_effect(move |_| {
         let mock = vec![
             DiffFile {
                 name: "src/main.rs".to_string(),
                 old_name: None,
                 index: "123".to_string(),
                 additions: 10,
                 deletions: 5,
                 type_: "modify".to_string(),
                 lines: vec![
                     DiffLine { line_no_old: Some(1), line_no_new: Some(1), content: " fn main() {".to_string(), type_: "context".to_string() },
                     DiffLine { line_no_old: Some(2), line_no_new: None, content: "-    println!(\"old\");".to_string(), type_: "delete".to_string() },
                     DiffLine { line_no_old: None, line_no_new: Some(2), content: "+    println!(\"new\");".to_string(), type_: "add".to_string() },
                 ],
             }
         ];
         set_diffs.set(mock);
    });

    view! {
        <div class="commit-diff">
            <h3>"Diff for " {owner} "/" {repo_name} " @ " {sha}</h3>
            <For each=move || diffs.get() key=|d| d.name.clone() children=move |d| {
                view! {
                    <div class="diff-file">
                        <h4>{d.name} " (" {d.additions} "+ / " {d.deletions} "-)"</h4>
                        <pre>
                            <For each=move || d.lines.clone() key=|l| (l.line_no_old, l.line_no_new) children=move |l| {
                                view! {
                                    <div class=format!("diff-line {}", l.type_)>
                                        <span>{l.line_no_old.map(|n| n.to_string()).unwrap_or_default()}</span>
                                        <span>{l.line_no_new.map(|n| n.to_string()).unwrap_or_default()}</span>
                                        <span>{l.content}</span>
                                    </div>
                                }
                            }/>
                        </pre>
                    </div>
                }
            }/>
        </div>
    }
}

#[component]
fn RepoCode() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());
    let path = move || params.with(|params| params.get("path").cloned().unwrap_or_default());

    let (files, set_files) = create_signal(vec![]);

    create_effect(move |_| {
        // Mock contents
        let mut mock = vec![];
        if path() == "" || path() == "/" {
            mock.push(FileEntry { name: "src".to_string(), path: "src".to_string(), kind: "dir".to_string(), size: 0 });
            mock.push(FileEntry { name: "README.md".to_string(), path: "README.md".to_string(), kind: "file".to_string(), size: 1024 });
        } else if path() == "src" {
             mock.push(FileEntry { name: "main.rs".to_string(), path: "src/main.rs".to_string(), kind: "file".to_string(), size: 512 });
        }
        set_files.set(mock);
    });

    view! {
        <div class="repo-code">
            <h3>"Code: " {owner} " / " {repo_name} " / " {path}</h3>
            <p><a href=format!("/api/v1/repos/{}/{}/raw/{}", owner(), repo_name(), path()) target="_blank">"View Raw"</a></p>
            <ul>
                <For
                    each=move || files.get()
                    key=|f| f.path.clone()
                    children=move |f| {
                        let href = format!("/repos/{}/{}/src/{}", owner(), repo_name(), f.path);
                        view! {
                            <li>
                                <a href=href>{f.name}</a> " (" {f.kind} ")"
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn IssueDetail() -> impl IntoView {
    let params = use_params_map();
    let index = move || params.with(|params| params.get("index").cloned().unwrap_or_default());

    let (comments, set_comments) = create_signal(vec![]);

    create_effect(move |_| {
        // Mock comments
        let user = User::new(1, "admin".to_string(), None);
        let mock = vec![
            Comment {
                id: 1,
                body: "Comment body".to_string(),
                user,
                created_at: "date".to_string(),
            }
        ];
        set_comments.set(mock);
    });

    view! {
        <div class="issue-detail">
             <h3>"Issue #" {index}</h3>
             <div class="comments">
                <For
                    each=move || comments.get()
                    key=|c| c.id
                    children=move |c| {
                        view! {
                            <div class="comment">
                                <strong>{c.user.username}</strong> ": " {c.body}
                                <div class="reactions">
                                    <button>"👍"</button>
                                </div>
                            </div>
                        }
                    }
                />
             </div>
             <form>
                <textarea placeholder="Add a comment"></textarea>
                <button>"Comment"</button>
             </form>
             <div class="sidebar">
                 <h4>"Assignees"</h4>
                 <button>"Add Assignee"</button>
                 <h4>"Labels"</h4>
                 <button>"Add Label"</button>
             </div>
        </div>
    }
}

#[component]
fn LabelList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (labels, set_labels) = create_signal(vec![]);
    create_effect(move |_| {
         // Mock
         let mock = vec![
             Label { id: 1, name: "bug".to_string(), color: "#f00".to_string(), description: None }
         ];
         set_labels.set(mock);
    });

    view! {
        <div class="label-list">
            <h3>"Labels for " {owner} " / " {repo_name}</h3>
            <ul>
                <For
                    each=move || labels.get()
                    key=|l| l.id
                    children=move |l| {
                        view! {
                            <li>
                                <span style=format!("background-color: {}", l.color)>{l.name}</span>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn MilestoneList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (milestones, set_milestones) = create_signal(vec![]);
    create_effect(move |_| {
         // Mock
         let mock = vec![
             Milestone { id: 1, title: "v1.0".to_string(), description: None, due_on: None, state: "open".to_string() }
         ];
         set_milestones.set(mock);
    });

    view! {
        <div class="milestone-list">
            <h3>"Milestones for " {owner} " / " {repo_name}</h3>
            <ul>
                <For
                    each=move || milestones.get()
                    key=|m| m.id
                    children=move |m| {
                        view! {
                            <li>
                                <a href=format!("/repos/{}/{}/milestones/{}", owner(), repo_name(), m.id)>
                                    <strong>{m.title}</strong>
                                </a> " (" {m.state} ")"
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn MilestoneDetail() -> impl IntoView {
    let params = use_params_map();
    let index = move || params.with(|params| params.get("index").cloned().unwrap_or_default());
    let (stats, set_stats) = create_signal(MilestoneStats { open_issues: 0, closed_issues: 0 });

    create_effect(move |_| {
        set_stats.set(MilestoneStats { open_issues: 10, closed_issues: 5 });
    });

    view! {
        <div class="milestone-detail">
            <h3>"Milestone " {index}</h3>
            <div class="stats">
                <span>{move || stats.get().open_issues} " Open"</span> " | "
                <span>{move || stats.get().closed_issues} " Closed"</span>
            </div>
            <div class="progress-bar" style="width: 100%; background: #eee; height: 10px;">
                <div style="width: 33%; background: green; height: 100%;"></div>
            </div>
        </div>
    }
}

#[component]
fn BranchList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (branches, set_branches) = create_signal(vec![]);
    create_effect(move |_| {
        let user = User::new(1, "admin".to_string(), None);
        let commit = Commit { sha: "abc".to_string(), message: "init".to_string(), author: user, date: "now".to_string() };
        set_branches.set(vec![
            Branch { name: "main".to_string(), commit, protected: true }
        ]);
    });

    view! {
        <div class="branch-list">
            <h3>"Branches for " {owner} " / " {repo_name}</h3>
            <p><a href="settings/branches">"Protection Rules"</a></p>
            <ul>
                <For
                    each=move || branches.get()
                    key=|b| b.name.clone()
                    children=move |b| {
                        view! {
                            <li>{b.name} " (" {if b.protected { "Protected" } else { "Regular" }} ")"</li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn TagList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (tags, set_tags) = create_signal(vec![]);
    create_effect(move |_| {
        let user = User::new(1, "admin".to_string(), None);
        let commit = Commit { sha: "abc".to_string(), message: "init".to_string(), author: user, date: "now".to_string() };
        set_tags.set(vec![
            Tag { name: "v1.0".to_string(), id: "1".to_string(), commit }
        ]);
    });

    view! {
        <div class="tag-list">
            <h3>"Tags for " {owner} " / " {repo_name}</h3>
            <ul>
                <For
                    each=move || tags.get()
                    key=|t| t.id.clone()
                    children=move |t| {
                        view! {
                            <li>{t.name}</li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn ProjectList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (projects, set_projects) = create_signal(vec![]);
    create_effect(move |_| {
         // Mock
         let mock = vec![
             Project { id: 1, title: "v1.0".to_string(), description: None, is_closed: false }
         ];
         set_projects.set(mock);
    });

    view! {
        <div class="project-list">
            <h3>"Projects for " {owner} " / " {repo_name}</h3>
            <ul>
                <For
                    each=move || projects.get()
                    key=|p| p.id
                    children=move |p| {
                        view! {
                            <li>
                                <strong>{p.title}</strong> " (" {if p.is_closed { "Closed" } else { "Open" }} ")"
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn TeamList() -> impl IntoView {
    let params = use_params_map();
    let org_name = move || params.with(|params| params.get("org").cloned().unwrap_or_default());

    let (teams, set_teams) = create_signal(vec![]);
    create_effect(move |_| {
         // Mock
         let mock = vec![
             Team { id: 1, name: "devs".to_string(), description: None, permission: "write".to_string() }
         ];
         set_teams.set(mock);
    });

    view! {
        <div class="team-list">
            <h3>"Teams for " {org_name}</h3>
            <ul>
                <For
                    each=move || teams.get()
                    key=|t| t.id
                    children=move |t| {
                        view! {
                            <li>
                                <strong>{t.name}</strong> " (" {t.permission} ")"
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn Wiki() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (page, set_page) = create_signal(None::<WikiPage>);

    create_effect(move |_| {
        // Mock
        set_page.set(Some(WikiPage {
            title: "Home".to_string(),
            content: "Welcome to the wiki!".to_string(),
            commit_message: None,
        }));
    });

    view! {
        <div class="wiki">
            <h3>"Wiki for " {owner} " / " {repo_name}</h3>
            {move || match page.get() {
                Some(p) => {
                    let edit_href = format!("/repos/{}/{}/wiki/pages/{}/edit", owner(), repo_name(), p.title);
                    view! {
                        <div>
                            <h4>{p.title} <a href=edit_href class="button">"Edit"</a></h4>
                            <p>{p.content}</p>
                        </div>
                    }.into_view()
                },
                None => view! { <p>"No page"</p> }.into_view()
            }}
        </div>
    }
}

#[component]
fn WikiEdit() -> impl IntoView {
    let params = use_params_map();
    let page_name = move || params.with(|params| params.get("page_name").cloned().unwrap_or_default());
    let (content, set_content) = create_signal("".to_string());

    let on_save = move |_| {
        let payload = CreateWikiPageOption { title: page_name(), content: content.get(), message: Some("Updated".to_string()) };
        leptos::logging::log!("Updating wiki: {:?}", payload);
    };

    view! {
        <div class="wiki-edit">
            <h3>"Editing " {page_name}</h3>
            <textarea prop:value=content on:input=move |ev| set_content.set(event_target_value(&ev))></textarea>
            <button on:click=on_save>"Save Page"</button>
        </div>
    }
}

#[component]
fn RepoCodeSearch() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());
    let (query, set_query) = create_signal("".to_string());
    let (results, set_results) = create_signal(vec![]);

    let on_search = move |_| {
        set_results.set(vec![
            CodeSearchResult { name: "main.rs".to_string(), path: "src/main.rs".to_string(), sha: "123".to_string(), url: "http://...".to_string(), content: Some("fn main".to_string()) }
        ]);
    };

    view! {
        <div class="repo-search">
            <h3>"Search in " {owner} "/" {repo_name}</h3>
            <input type="text" placeholder="Search code..." prop:value=query on:input=move |ev| set_query.set(event_target_value(&ev)) />
            <button on:click=on_search>"Search"</button>
            <ul>
                <For each=move || results.get() key=|r| r.sha.clone() children=move |r| {
                    view! {
                        <li>
                            <a href=r.url>{r.path}</a>
                            <pre>{r.content.unwrap_or_default()}</pre>
                        </li>
                    }
                }/>
            </ul>
        </div>
    }
}

#[component]
fn RepoSettings() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (desc, set_desc) = create_signal("".to_string());

    let on_save = move |_| {
        let payload = RepoSettingsOption {
            description: Some(desc.get()),
            private: None,
            website: None,
        };
        leptos::logging::log!("Saving settings: {:?}", payload);
    };

    view! {
        <div class="repo-settings">
            <h3>"Settings for " {owner} " / " {repo_name}</h3>
            <input type="text" placeholder="Description" prop:value=desc
                on:input=move |ev| set_desc.set(event_target_value(&ev)) />
            <button on:click=on_save>"Save"</button>
            <Webhooks/>
            <SecretList/>
            <DeployKeyList/>
            <MirrorSettings/>
            <CollaboratorList/>
            <LfsSettings/>
            <TransferSettings/>
        </div>
    }
}

#[component]
fn TransferSettings() -> impl IntoView {
    let (new_owner, set_new_owner) = create_signal("".to_string());

    let on_transfer = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = TransferRepoOption { new_owner: new_owner.get() };
        leptos::logging::log!("Transferring repo: {:?}", payload);
    };

    view! {
        <div class="transfer-settings">
            <h4>"Transfer Ownership"</h4>
            <form on:submit=on_transfer>
                <input type="text" placeholder="New Owner Username" prop:value=new_owner on:input=move |ev| set_new_owner.set(event_target_value(&ev)) />
                <button type="submit" class="danger">"Transfer"</button>
            </form>
        </div>
    }
}

#[component]
fn LfsSettings() -> impl IntoView {
    view! {
        <div class="lfs-settings">
            <h4>"LFS Settings"</h4>
            <p>"LFS is enabled for this repository."</p>
        </div>
    }
}

#[component]
fn CollaboratorList() -> impl IntoView {
    let (collabs, set_collabs) = create_signal(vec![]);
    create_effect(move |_| {
        set_collabs.set(vec![
            Collaborator { user: User::new(3, "collab".to_string(), None), permissions: "write".to_string() }
        ]);
    });

    view! {
        <div class="collaborators">
            <h4>"Collaborators"</h4>
            <ul>
                <For
                    each=move || collabs.get()
                    key=|c| c.user.id
                    children=move |c| {
                        view! {
                            <li>{c.user.username} " (" {c.permissions} ")"</li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn MirrorSettings() -> impl IntoView {
    let on_sync = move |_| {
        leptos::logging::log!("Syncing mirror...");
    };

    view! {
        <div class="mirror-settings">
            <h4>"Mirror Settings"</h4>
            <button on:click=on_sync>"Sync Now"</button>
        </div>
    }
}

#[component]
fn SecretList() -> impl IntoView {
    let (secrets, set_secrets) = create_signal(vec![]);
    create_effect(move |_| {
        set_secrets.set(vec![
            Secret { name: "CI_TOKEN".to_string(), created_at: "now".to_string() }
        ]);
    });

    view! {
        <div class="secrets">
            <h4>"Secrets"</h4>
            <ul>
                <For
                    each=move || secrets.get()
                    key=|s| s.name.clone()
                    children=move |s| {
                        view! {
                            <li>{s.name}</li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn DeployKeyList() -> impl IntoView {
    let (keys, set_keys) = create_signal(vec![]);
    create_effect(move |_| {
        set_keys.set(vec![
            DeployKey { id: 1, title: "Deploy Key".to_string(), key: "ssh...".to_string(), fingerprint: "f".to_string() }
        ]);
    });

    view! {
        <div class="deploy-keys">
            <h4>"Deploy Keys"</h4>
            <ul>
                <For
                    each=move || keys.get()
                    key=|k| k.id
                    children=move |k| {
                        view! {
                            <li>{k.title} " - " {k.fingerprint}</li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn Webhooks() -> impl IntoView {
    let (hooks, set_hooks) = create_signal(vec![]);
    create_effect(move |_| {
        // Mock
        set_hooks.set(vec![
            Webhook { id: 1, url: "http://example.com".to_string(), events: vec!["push".to_string()], active: true }
        ]);
    });

    view! {
        <div class="webhooks">
            <h4>"Webhooks"</h4>
            <ul>
                <For
                    each=move || hooks.get()
                    key=|h| h.id
                    children=move |h| {
                        view! {
                            <li>{h.url} " (" {if h.active { "Active" } else { "Inactive" }} ")"</li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn UserSettings() -> impl IntoView {
    let (name, set_name) = create_signal("".to_string());

    let on_save = move |_| {
        let payload = UserSettingsOption {
            full_name: Some(name.get()),
            website: None,
            description: None,
            location: None,
        };
        leptos::logging::log!("Saving user settings: {:?}", payload);
    };

    view! {
        <div class="user-settings">
            <h3>"User Settings"</h3>
            <input type="text" placeholder="Full Name" prop:value=name
                on:input=move |ev| set_name.set(event_target_value(&ev)) />
            <button on:click=on_save>"Save"</button>
            <SSHKeys/>
            <GpgKeys/>
            <TwoFactorSettings/>
            <EmailSettings/>
            <AppSettings/>
        </div>
    }
}

#[component]
fn EmailSettings() -> impl IntoView {
    let (emails, set_emails) = create_signal(vec![]);
    create_effect(move |_| {
        set_emails.set(vec![
            EmailAddress { email: "user@example.com".to_string(), verified: true, primary: true }
        ]);
    });

    view! {
        <div class="email-settings">
            <h4>"Email Addresses"</h4>
            <ul>
                <For each=move || emails.get() key=|e| e.email.clone() children=move |e| {
                    view! { <li>{e.email} {if e.primary { "(Primary)" } else { "" }} {if e.verified { "(Verified)" } else { "(Unverified)" }}</li> }
                }/>
            </ul>
        </div>
    }
}

#[component]
fn AppSettings() -> impl IntoView {
    let (apps, set_apps) = create_signal(vec![]);
    create_effect(move |_| {
        set_apps.set(vec![
            OAuth2Application { id: 1, name: "App1".to_string(), client_id: "id".to_string(), redirect_uris: vec![] }
        ]);
    });

    view! {
        <div class="app-settings">
            <h4>"Applications"</h4>
            <ul>
                <For each=move || apps.get() key=|a| a.id children=move |a| {
                    view! { <li>{a.name} " (" {a.client_id} ")"</li> }
                }/>
            </ul>
        </div>
    }
}

#[component]
fn GpgKeys() -> impl IntoView {
    let (keys, set_keys) = create_signal(vec![]);
    create_effect(move |_| {
        set_keys.set(vec![
            GpgKey { id: 1, key_id: "ID".to_string(), primary_key_id: "PID".to_string(), public_key: "PUB".to_string(), emails: vec![] }
        ]);
    });

    view! {
        <div class="gpg-keys">
            <h4>"GPG Keys"</h4>
            <ul>
                <For
                    each=move || keys.get()
                    key=|k| k.id
                    children=move |k| {
                        view! {
                            <li>{k.key_id} <button>"Delete"</button></li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn TwoFactorSettings() -> impl IntoView {
    let (status, set_status) = create_signal(None::<TwoFactor>);
    create_effect(move |_| {
        set_status.set(Some(TwoFactor { enabled: false, method: "totp".to_string() }));
    });

    view! {
        <div class="2fa-settings">
            <h4>"Two-Factor Authentication"</h4>
            {move || match status.get() {
                Some(s) => view! {
                    <p>"Status: " {if s.enabled { "Enabled" } else { "Disabled" }} " (" {s.method} ")"</p>
                }.into_view(),
                None => view! { <p>"Loading..."</p> }.into_view()
            }}
        </div>
    }
}

#[component]
fn SSHKeys() -> impl IntoView {
    let (keys, set_keys) = create_signal(vec![]);
    create_effect(move |_| {
        // Mock
        set_keys.set(vec![
            PublicKey { id: 1, title: "My Key".to_string(), key: "ssh-rsa...".to_string(), fingerprint: "SHA...".to_string() }
        ]);
    });

    view! {
        <div class="ssh-keys">
            <h4>"SSH Keys"</h4>
            <ul>
                <For
                    each=move || keys.get()
                    key=|k| k.id
                    children=move |k| {
                        view! {
                            <li>{k.title} " - " {k.fingerprint} <button>"Delete"</button></li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn PullRequestDetail() -> impl IntoView {
    let params = use_params_map();
    let index = move || params.with(|params| params.get("index").cloned().unwrap_or_default());

    let on_merge = move |_| {
        leptos::logging::log!("Merging PR #{}", index());
    };

    view! {
        <div class="pr-detail">
            <h3>"Pull Request #" {index}</h3>
            <div class="sidebar">
                <h4>"Reviewers"</h4>
                <button>"Request Review"</button>
            </div>
            <div class="tabs">
                <button>"Conversation"</button>
                <button>"Commits"</button>
                <button>"Files Changed"</button>
            </div>
            <button on:click=on_merge>"Merge Pull Request"</button>
        </div>
    }
}

#[component]
fn PullRequestList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (pulls, set_pulls) = create_signal(vec![]);

    create_effect(move |_| {
        // Mock fetch
        let user = User::new(1, "admin".to_string(), None);
        let mock = vec![
             PullRequest {
                id: 1,
                number: 1,
                title: "First PR".to_string(),
                body: Some("Desc".to_string()),
                state: "open".to_string(),
                user,
                merged: false,
            }
        ];
        set_pulls.set(mock);
    });

    view! {
        <div class="pull-list">
             <h3>"Pull Requests for " {owner} " / " {repo_name}</h3>
             <ul>
                <For
                    each=move || pulls.get()
                    key=|pr| pr.id
                    children=move |pr| {
                        let href = format!("/repos/{}/{}/pulls/{}", owner(), repo_name(), pr.number);
                        view! {
                            <li>
                                <a href=href><strong>"#" {pr.number} " " {pr.title}</strong></a>
                                " (" {pr.state} ") by " {pr.user.username}
                            </li>
                        }
                    }
                />
             </ul>
        </div>
    }
}

#[component]
fn IssueList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (issues, set_issues) = create_signal(vec![]);

    create_effect(move |_| {
        // Mock fetch
        let user = User::new(1, "admin".to_string(), None);
        let mock = vec![
             Issue {
                id: 1,
                number: 1,
                title: "First Issue".to_string(),
                body: Some("Bug report".to_string()),
                state: "open".to_string(),
                user,
                assignees: vec![],
            }
        ];
        set_issues.set(mock);
    });

    view! {
        <div class="issue-list">
             <h3>"Issues for " {owner} " / " {repo_name}</h3>
             <ul>
                <For
                    each=move || issues.get()
                    key=|issue| issue.id
                    children=move |issue| {
                        let href = format!("/repos/{}/{}/issues/{}", owner(), repo_name(), issue.number);
                        view! {
                            <li>
                                <a href=href><strong>"#" {issue.number} " " {issue.title}</strong></a>
                                " (" {issue.state} ") by " {issue.user.username}
                            </li>
                        }
                    }
                />
             </ul>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_home_logic() {
        // Simple verification that types align
        let repo = Repository::new(1, "test".to_string(), "me".to_string());
        assert_eq!(repo.name, "test");
    }

    #[test]
    fn test_gpg_mirror_logic() {
        let k = GpgKey { id: 1, key_id: "id".to_string(), primary_key_id: "p".to_string(), public_key: "k".to_string(), emails: vec![] };
        assert_eq!(k.key_id, "id");
    }

    #[test]
    fn test_create_repo_logic() {
        let opts = CreateRepoOption {
            name: "test".to_string(),
            description: None,
            private: false,
            auto_init: true,
        };
        assert_eq!(opts.name, "test");
    }

    #[test]
    fn test_issue_logic() {
        let user = User::new(1, "u".to_string(), None);
        let issue = Issue {
            id: 1,
            number: 1,
            title: "t".to_string(),
            body: None,
            state: "o".to_string(),
            user,
            assignees: vec![],
        };
        assert_eq!(issue.title, "t");
    }

    #[test]
    fn test_pr_logic() {
        let user = User::new(1, "u".to_string(), None);
        let pr = PullRequest {
            id: 1,
            number: 1,
            title: "p".to_string(),
            body: None,
            state: "o".to_string(),
            user,
            merged: false,
        };
        assert_eq!(pr.title, "p");
    }

    #[test]
    fn test_file_entry_logic() {
        let f = FileEntry {
            name: "n".to_string(),
            path: "p".to_string(),
            kind: "f".to_string(),
            size: 1,
        };
        assert_eq!(f.name, "n");
    }

    #[test]
    fn test_commit_logic() {
        let user = User::new(1, "u".to_string(), None);
        let c = Commit {
            sha: "s".to_string(),
            message: "m".to_string(),
            author: user,
            date: "d".to_string(),
        };
        assert_eq!(c.sha, "s");
    }

    #[test]
    fn test_release_logic() {
        let user = User::new(1, "u".to_string(), None);
        let r = Release {
            id: 1,
            tag_name: "v".to_string(),
            name: "n".to_string(),
            body: None,
            draft: false,
            prerelease: false,
            created_at: "d".to_string(),
            author: user,
        };
        assert_eq!(r.tag_name, "v");
    }

    #[test]
    fn test_auth_logic() {
        let l = LoginOption {
            username: "u".to_string(),
            password: "p".to_string(),
        };
        assert_eq!(l.username, "u");
    }

    #[test]
    fn test_org_logic() {
        let o = Organization {
            id: 1,
            username: "o".to_string(),
            description: None,
            avatar_url: None,
        };
        assert_eq!(o.username, "o");
    }

    #[test]
    fn test_comment_logic() {
        let user = User::new(1, "u".to_string(), None);
        let c = Comment {
            id: 1,
            body: "b".to_string(),
            user,
            created_at: "d".to_string(),
        };
        assert_eq!(c.body, "b");
    }

    #[test]
    fn test_label_logic() {
        let l = Label { id: 1, name: "l".to_string(), color: "c".to_string(), description: None };
        assert_eq!(l.name, "l");
    }

    #[test]
    fn test_topic_logic() {
        let t = Topic { id: 1, name: "t".to_string(), created: "d".to_string() };
        assert_eq!(t.name, "t");
    }

    #[test]
    fn test_wiki_logic() {
        let p = WikiPage { title: "t".to_string(), content: "c".to_string(), commit_message: None };
        assert_eq!(p.title, "t");
    }

    #[test]
    fn test_key_logic() {
        let k = PublicKey { id: 1, title: "t".to_string(), key: "k".to_string(), fingerprint: "f".to_string() };
        assert_eq!(k.title, "t");
    }

    #[test]
    fn test_team_logic() {
        let t = Team { id: 1, name: "t".to_string(), description: None, permission: "p".to_string() };
        assert_eq!(t.name, "t");
    }

    #[test]
    fn test_admin_logic() {
        let s = AdminStats { users: 1, repos: 2, orgs: 3, issues: 4 };
        assert_eq!(s.users, 1);
    }

    #[test]
    fn test_activity_logic() {
        let a = Activity {
            id: 1,
            user_id: 1,
            user_name: "u".to_string(),
            op_type: "op".to_string(),
            content: "c".to_string(),
            created: "t".to_string(),
        };
        assert_eq!(a.op_type, "op");
    }

    #[test]
    fn test_actions_pkg_logic() {
        let wf = ActionWorkflow { id: 1, name: "w".to_string(), status: "s".to_string() };
        assert_eq!(wf.name, "w");

        let p = Package { id: 1, name: "p".to_string(), version: "v".to_string(), package_type: "t".to_string() };
        assert_eq!(p.name, "p");
    }

    #[test]
    fn test_secret_logic() {
        let s = Secret { name: "n".to_string(), created_at: "t".to_string() };
        assert_eq!(s.name, "n");
        let k = DeployKey { id: 1, title: "t".to_string(), key: "k".to_string(), fingerprint: "f".to_string() };
        assert_eq!(k.title, "t");
    }

    #[test]
    fn test_notice_2fa_logic() {
        let n = SystemNotice { id: 1, type_: "t".to_string(), description: "d".to_string() };
        assert_eq!(n.type_, "t");
        let tf = TwoFactor { enabled: true, method: "m".to_string() };
        assert!(tf.enabled);
    }

    #[test]
    fn test_lfs_oauth_structs() {
        let lfs = LfsObject { oid: "o".to_string(), size: 10, created_at: "t".to_string() };
        assert_eq!(lfs.size, 10);
        let oa = OAuth2Provider { name: "n".to_string(), display_name: "d".to_string(), url: "u".to_string() };
        assert_eq!(oa.name, "n");
    }

    #[test]
    fn test_reaction_struct() {
        let u = User::new(1, "u".to_string(), None);
        let r = Reaction { id: 1, user: u, content: "c".to_string(), created_at: "t".to_string() };
        assert_eq!(r.content, "c");
    }

    #[test]
    fn test_diff_components() {
        let line = DiffLine { line_no_old: None, line_no_new: Some(1), content: "c".to_string(), type_: "add".to_string() };
        assert_eq!(line.content, "c");
    }

    #[test]
    fn test_heatmap_org_logic() {
        let c = Contribution { date: "d".to_string(), count: 1 };
        assert_eq!(c.count, 1);
        let m = OrgMember { user: User::new(1, "u".to_string(), None), role: "r".to_string() };
        assert_eq!(m.role, "r");
    }

    #[test]
    fn test_template_ui_logic() {
        let l = LicenseTemplate { key: "k".to_string(), name: "n".to_string(), url: "u".to_string() };
        assert_eq!(l.name, "n");
    }

    #[test]
    fn test_admin_review_ui() {
        let u = User::new(1, "u".to_string(), None);
        let rr = ReviewRequest { reviewer: u, status: "s".to_string() };
        assert_eq!(rr.status, "s");
        let ae = AdminUserEditOption { email: None, password: None, active: None, admin: Some(true) };
        assert!(ae.admin.unwrap());
    }

    #[test]
    fn test_new_settings_ui() {
        let l = LanguageStat { language: "L".to_string(), percentage: 50, color: "c".to_string() };
        assert_eq!(l.percentage, 50);
        let e = EmailAddress { email: "e".to_string(), verified: true, primary: false };
        assert!(!e.primary);
    }

    #[test]
    fn test_migrate_transfer_ui() {
        let m = MigrateRepoOption { clone_addr: "u".to_string(), repo_name: "n".to_string(), service: "s".to_string(), mirror: false };
        assert!(!m.mirror);
        let t = TransferRepoOption { new_owner: "o".to_string() };
        assert_eq!(t.new_owner, "o");
    }

    #[test]
    fn test_milestone_detail_ui() {
        let s = MilestoneStats { open_issues: 1, closed_issues: 0 };
        assert_eq!(s.open_issues, 1);
    }

    #[test]
    fn test_search_package_wiki_ui() {
        let r = CodeSearchResult { name: "n".to_string(), path: "p".to_string(), sha: "s".to_string(), url: "u".to_string(), content: None };
        assert_eq!(r.name, "n");
    }
}

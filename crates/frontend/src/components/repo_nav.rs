use leptos::*;
use leptos_router::*;

/// Secondary navigation shown on every repository page.
///
/// Keeps the repository section links consistent instead of repeating a long
/// inline list on each page. The link matching the current route is marked
/// with `aria-current="page"` and the `active` class.
#[component]
pub fn RepoNav() -> impl IntoView {
    let params = use_params_map();
    let location = use_location();
    let owner = move || params.with(|p| p.get("owner").cloned().unwrap_or_default());
    let repo = move || params.with(|p| p.get("repo").cloned().unwrap_or_default());
    let base = move || format!("/repos/{}/{}", owner(), repo());

    let links = [
        ("", "Code"),
        ("/issues", "Issues"),
        ("/pulls", "Pull requests"),
        ("/commits", "Commits"),
        ("/branches", "Branches"),
        ("/tags", "Tags"),
        ("/releases", "Releases"),
        ("/labels", "Labels"),
        ("/milestones", "Milestones"),
        ("/wiki", "Wiki"),
        ("/projects", "Projects"),
        ("/discussions", "Discussions"),
        ("/actions", "Actions"),
        ("/pulse", "Pulse"),
        ("/settings", "Settings"),
    ];

    let is_active = move |suffix: &str| {
        let path = location.pathname.get();
        let root = base();
        if suffix.is_empty() {
            // The code browser lives both at the repo root and under /src.
            path == root || path == format!("{root}/") || path.starts_with(&format!("{root}/src"))
        } else {
            path == format!("{root}{suffix}") || path.starts_with(&format!("{root}{suffix}/"))
        }
    };

    view! {
        <nav class="repo-nav">
            {links.iter().map(|(suffix, label)| {
                let (suffix, label) = (*suffix, *label);
                let href = move || format!("{}{}", base(), suffix);
                let active = move || is_active(suffix);
                view! {
                    <a href=href class:active=active aria-current=move || active().then_some("page")>
                        {label}
                    </a>
                }
            }).collect_view()}
        </nav>
    }
}

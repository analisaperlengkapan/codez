use leptos::*;
use shared::Repository;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (repos, set_repos) = create_signal(vec![]);

    // Mock fetching data for CSR
    create_effect(move |_| {
        let mock_repos = vec![
            Repository::new(1, "codeza-frontend".to_string(), "admin".to_string()),
        ];
        set_repos.set(mock_repos);
    });

    view! {
        <div class="container">
            <h1>"Codeza Repositories"</h1>
            <ul>
                <For
                    each=move || repos.get()
                    key=|repo| repo.id
                    children=move |repo| {
                        view! {
                            <li>
                                <strong>{repo.name}</strong> " by " {repo.owner}
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
    fn test_app_logic() {
        let repo = Repository::new(1, "test".to_string(), "me".to_string());
        assert_eq!(repo.name, "test");
    }
}

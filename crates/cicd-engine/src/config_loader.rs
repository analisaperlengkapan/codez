use codeza_git_service::GitProvider;

pub async fn load_yaml_pipeline_config(
    provider: &dyn GitProvider,
    repo_full_name: &str,
    git_ref: &str,
) -> Option<String> {
    // Try to load .codeza.yml or .codeza.yaml
    let paths = [".codeza.yml", ".codeza.yaml"];

    let (owner, repo_name) = match repo_full_name.split_once('/') {
        Some((o, r)) => (o, r),
        None => {
            tracing::error!("Invalid repo full name: {}", repo_full_name);
            return None;
        }
    };

    for path in paths {
        match provider
            .get_file_contents(owner, repo_name, path, git_ref)
            .await
        {
            Ok(content) => return Some(content),
            Err(_) => continue,
        }
    }

    None
}

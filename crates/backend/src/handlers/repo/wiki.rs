use super::*;

pub async fn create_wiki_page(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreateWikiPageOption>,
) -> (StatusCode, Json<WikiPage>) {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return (
            StatusCode::NOT_FOUND,
            Json(WikiPage {
                title: "".to_string(),
                content: "".to_string(),
                commit_message: None,
            }),
        );
    }

    let mut wikis = state.wikis.write().unwrap_or_else(|e| e.into_inner());
    let page = WikiPage {
        title: payload.title.clone(),
        content: payload.content.clone(),
        commit_message: payload.message.clone(),
    };
    wikis.insert((repo_id, payload.title), page.clone());

    (StatusCode::CREATED, Json(page))
}

pub async fn list_wiki_pages(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Json<Vec<WikiPage>> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return Json(vec![]);
    }

    let wikis = state.wikis.read().unwrap_or_else(|e| e.into_inner());
    let filtered_pages: Vec<WikiPage> = wikis
        .iter()
        .filter(|(&(r_id, _), _)| r_id == repo_id)
        .map(|(_, page)| page.clone())
        .collect();

    Json(filtered_pages)
}

pub async fn update_wiki_page(
    State(state): State<AppState>,
    Path((owner, repo_name, page_name)): Path<(String, String, String)>,
    Json(payload): Json<CreateWikiPageOption>,
) -> StatusCode {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return StatusCode::NOT_FOUND;
    }

    let mut wikis = state.wikis.write().unwrap_or_else(|e| e.into_inner());
    if wikis.contains_key(&(repo_id, page_name.clone())) {
        let old_key = (repo_id, page_name.clone());
        let new_key = (repo_id, payload.title.clone());
        let page = WikiPage {
            title: payload.title,
            content: payload.content,
            commit_message: payload.message,
        };

        if old_key != new_key {
            wikis.remove(&old_key);
        }
        wikis.insert(new_key, page);

        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn get_wiki_page(
    State(state): State<AppState>,
    Path((owner, repo_name, page_name)): Path<(String, String, String)>,
) -> Json<Option<WikiPage>> {
    let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    if repo_id == 0 {
        return Json(None);
    }

    let wikis = state.wikis.read().unwrap_or_else(|e| e.into_inner());
    if let Some(page) = wikis.get(&(repo_id, page_name)) {
        Json(Some(page.clone()))
    } else {
        Json(None)
    }
}

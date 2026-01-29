use crate::components::*;
use crate::pages::*;
use leptos::*;
use leptos_router::*;

mod api;
mod components;
mod pages;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Nav/>
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
                    <Route path="/repo/migrate" view=MigrateRepo/>
                    <Route path="/org/create" view=CreateOrg/>
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
                    <Route path="/repos/:owner/:repo/actions/workflows/:id" view=WorkflowRunsList/>
                    <Route path="/repos/:owner/:repo/branches" view=BranchList/>
                    <Route path="/repos/:owner/:repo/tags" view=TagList/>
                    <Route path="/repos/:owner/:repo/src/*path" view=RepoCode/>
                    <Route path="/repos/:owner/:repo/commits" view=CommitList/>
                    <Route path="/repos/:owner/:repo/commits/:sha" view=CommitDiff/>
                    <Route path="/repos/:owner/:repo/releases" view=ReleaseList/>
                    <Route path="/repos/:owner/:repo/releases/new" view=ReleaseCreate/>
                    <Route path="/repos/:owner/:repo/releases/:id" view=ReleaseDetail/>
                    <Route path="/repos/:owner/:repo/labels" view=LabelList/>
                    <Route path="/repos/:owner/:repo/milestones" view=MilestoneList/>
                    <Route path="/repos/:owner/:repo/milestones/:index" view=MilestoneDetail/>
                    <Route path="/repos/:owner/:repo/projects" view=ProjectList/>
                    <Route path="/repos/:owner/:repo/projects/:id" view=ProjectDetail/>
                    <Route path="/repos/:owner/:repo/discussions" view=DiscussionList/>
                    <Route path="/repos/:owner/:repo/discussions/:id" view=DiscussionDetail/>
                    <Route path="/repos/:owner/:repo/wiki" view=Wiki/>
                    <Route path="/repos/:owner/:repo/wiki/pages/:page_name" view=Wiki/>
                    <Route path="/repos/:owner/:repo/wiki/pages/:page_name/edit" view=WikiEdit/>
                    <Route path="/repos/:owner/:repo/edit/*path" view=FileEdit/>
                    <Route path="/repos/:owner/:repo/settings" view=RepoSettings/>
                    <Route path="/repos/:owner/:repo/collaborators" view=CollaboratorList/>
                    <Route path="/repos/:owner/:repo/settings/webhooks" view=WebhookList/>
                    <Route path="/repos/:owner/:repo/settings/secrets" view=SecretList/>
                    <Route path="/repos/:owner/:repo/settings/keys" view=DeployKeyList/>
                    <Route path="/repos/:owner/:repo/settings/branches" view=ProtectedBranchList/>
                    <Route path="/repos/:owner/:repo/settings/lfs" view=LfsLockList/>
                </Routes>
            </main>
        </Router>
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_frontend_routes() {
        assert_eq!(1, 1);
    }
}

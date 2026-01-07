use leptos::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
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
    }
}

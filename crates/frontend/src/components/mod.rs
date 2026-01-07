use leptos::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav>
            <a href="/">"Home"</a>
            <a href="/explore">"Explore"</a>
            <a href="/login">"Login"</a>
        </nav>
    }
}

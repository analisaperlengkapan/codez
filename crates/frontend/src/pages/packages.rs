use leptos::*;
use gloo_net::http::Request;
use leptos_router::*;
use shared::{Package, CreatePackageOption};

#[component]
pub fn PackageList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());

    let (show_upload, set_show_upload) = create_signal(false);
    let (refresh, set_refresh) = create_signal(0);

    let packages = create_resource(
        move || (owner(), refresh.get()),
        |(owner_name, _)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/packages/{}", owner_name)).send().await.unwrap().json::<Vec<Package>>().await.unwrap_or_default()
        }
    );

    view! {
        <div class="package-list">
            <div class="header" style="display: flex; justify-content: space-between; align-items: center;">
                <h3>"Packages for " {owner}</h3>
                <button on:click=move |_| set_show_upload.set(!show_upload.get())>
                    {move || if show_upload.get() { "Cancel" } else { "Upload Package" }}
                </button>
            </div>

            {move || if show_upload.get() {
                view! { <UploadPackageForm owner=owner() on_success=move || { set_show_upload.set(false); set_refresh.update(|n| *n += 1); } /> }.into_view()
            } else {
                view! { <span></span> }.into_view()
            }}

            <ul>
                <Suspense fallback=move || view! { <li>"Loading..."</li> }>
                    {move || packages.get().map(|list| view! {
                        <For each=move || list.clone() key=|p| p.id children=move |p| {
                            let href = format!("/packages/{}/{}/{}/{}", owner(), p.package_type, p.name, p.version);
                            view! {
                                <li style="border: 1px solid #eee; padding: 10px; margin-bottom: 5px;">
                                    <a href=href style="font-weight: bold;">{p.name}</a>
                                    <span style="margin-left: 10px; color: #666;">"v" {p.version} " (" {p.package_type} ")"</span>
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
fn UploadPackageForm<F>(owner: String, on_success: F) -> impl IntoView
where F: Fn() + Clone + 'static
{
    let (name, set_name) = create_signal("".to_string());
    let (version, set_version) = create_signal("".to_string());
    let (pkg_type, set_pkg_type) = create_signal("npm".to_string());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let payload = CreatePackageOption {
            name: name.get(),
            version: version.get(),
            package_type: pkg_type.get(),
        };
        let o = owner.clone();
        let on_success_clone = on_success.clone();
        spawn_local(async move {
            let res = Request::post(&format!("http://127.0.0.1:3000/api/v1/packages/{}", o))
                .json(&payload).unwrap().send().await;
            if let Ok(r) = res {
                if r.ok() {
                    on_success_clone();
                }
            }
        });
    };

    view! {
        <form on:submit=on_submit style="background: #f9f9f9; padding: 10px; margin-bottom: 10px; border: 1px solid #ddd;">
            <div style="margin-bottom: 5px;">
                <input type="text" placeholder="Package Name" prop:value=name on:input=move |ev| set_name.set(event_target_value(&ev)) required />
            </div>
            <div style="margin-bottom: 5px;">
                <input type="text" placeholder="Version (e.g. 1.0.0)" prop:value=version on:input=move |ev| set_version.set(event_target_value(&ev)) required />
            </div>
            <div style="margin-bottom: 5px;">
                <select on:change=move |ev| set_pkg_type.set(event_target_value(&ev))>
                    <option value="npm">"npm"</option>
                    <option value="maven">"Maven"</option>
                    <option value="cargo">"Cargo"</option>
                    <option value="docker">"Docker"</option>
                    <option value="generic">"Generic"</option>
                </select>
            </div>
            <button type="submit">"Upload"</button>
        </form>
    }
}

#[component]
pub fn PackageDetail() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let name = move || params.with(|params| params.get("name").cloned().unwrap_or_default());
    let version = move || params.with(|params| params.get("version").cloned().unwrap_or_default());
    let pkg_type = move || params.with(|params| params.get("type").cloned().unwrap_or_default());

    let package = create_resource(
        move || (owner(), pkg_type(), name(), version()),
        |(o, t, n, v)| async move {
            Request::get(&format!("http://127.0.0.1:3000/api/v1/packages/{}/{}/{}/{}", o, t, n, v))
                .send().await.unwrap().json::<Option<Package>>().await.unwrap_or(None)
        }
    );

    view! {
        <div class="package-detail">
            <Suspense fallback=move || view! { <h3>"Loading..."</h3> }>
                {move || match package.get() {
                    Some(Some(p)) => {
                        let name = p.name.clone();
                        let owner = p.owner.clone();
                        let version = p.version.clone();
                        let pkg_type = p.package_type.clone();
                        let install_cmd = match pkg_type.as_str() {
                                    "npm" => format!("npm install {}@{}", name, version),
                                    "cargo" => format!("cargo add {}@{}", name, version),
                                    _ => "See documentation".to_string()
                        };

                        view! {
                            <h3>"Package: " {name}</h3>
                            <div class="meta">
                                <p><strong>"Owner:"</strong> " " {owner}</p>
                                <p><strong>"Version:"</strong> " " {version}</p>
                                <p><strong>"Type:"</strong> " " {pkg_type}</p>
                            </div>
                            <div class="install-instructions" style="background: #eee; padding: 10px; margin-top: 20px;">
                                <h4>"Installation"</h4>
                                <pre>{install_cmd}</pre>
                            </div>
                        }.into_view()
                    },
                    _ => view! { <h3>"Package Not Found"</h3> }.into_view()
                }}
            </Suspense>
        </div>
    }
}

use crate::components::RepoNav;
use gloo_net::http::Request;
use leptos::*;
use leptos_router::*;
use shared::SecurityScanReport;

#[component]
pub fn SecurityDashboard() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let (scanning, set_scanning) = create_signal(false);
    let (refresh, set_refresh) = create_signal(0);

    let report = create_resource(
        move || (owner(), repo_name(), refresh.get()),
        |(o, r, _)| async move {
            Request::get(&format!("/api/v1/repos/{}/{}/security/scan", o, r))
                .send()
                .await
                .unwrap()
                .json::<SecurityScanReport>()
                .await
                .ok()
        },
    );

    let on_run_scan = move |_| {
        let o = owner();
        let r = repo_name();
        set_scanning.set(true);
        spawn_local(async move {
            let _ = Request::post(&format!("/api/v1/repos/{}/{}/security/scan", o, r))
                .send()
                .await;
            set_scanning.set(false);
            set_refresh.update(|n| *n += 1);
        });
    };

    view! {
            <div class="security-dashboard">
            <RepoNav/>
                <div class="security-header">
                    <h2>"🛡️ Security Audit & Vulnerability Scanner"</h2>
                    <button
                        class="btn-primary"
                        on:click=on_run_scan
                        disabled=move || scanning.get()
    >
                        {move || if scanning.get() { "Scanning..." } else { "Run Security Audit" }}
                    </button>
                </div>

                <Suspense fallback=move || view! { <div>"Loading security report..."</div> }>
                    {move || report.get().map(|rep| match rep {
                        Some(rep) => {
                            let score_color = if rep.score>= 80 { "#2da44e" } else if rep.score>= 50 { "#d97706" } else { "#cf222e" };
                            view! {
                                <div>
                                    <div class="security-score">
                                        <div class="score-ring" style=format!("color: {score_color}; border-color: {score_color}")>
                                            {rep.score}
                                        </div>
                                        <div>
                                            <h3 class="mt-0">"Security Health Score: " {rep.score} "/100"</h3>
                                            <p class="text-small text-muted mt-1">"Scanned at: " {rep.scanned_at}</p>
                                        </div>
                                    </div>

                                    <h3>"Vulnerabilities Found (" {rep.vulnerabilities.len()} ")"</h3>
                                    {if rep.vulnerabilities.is_empty() {
                                        view! {
                                            <div class="security-clear">
                                                "✅ No vulnerabilities or secret leaks detected in repository!"
                                            </div>
                                        }.into_view()
                                    } else {
                                        view! {
                                            <div class="vulnerability-list">
                                                <For each=move || rep.vulnerabilities.clone() key=|v| v.id.clone() children=move |v| {
                                                    let badge_bg = match v.severity.as_str() {
                                                        "CRITICAL" => "#ffebe9",
                                                        "HIGH" => "#fff8c5",
                                                        _ => "#ddf4ff",
                                                    };
                                                    let badge_color = match v.severity.as_str() {
                                                        "CRITICAL" => "#cf222e",
                                                        "HIGH" => "#9a6700",
                                                        _ => "#0969da",
                                                    };
                                                    view! {
                                                        <div class="vulnerability" style=format!("background: {badge_bg}")>
                                                            <div class="flex-between">
                                                                <strong style=format!("color: {badge_color}")>
                                                                    "[" {v.severity} "] " {v.title}
                                                                </strong>
                                                                <span class="text-small text-muted">{v.id}</span>
                                                            </div>
                                                            <p class="vulnerability-desc">{v.description}</p>
                                                            <div class="vulnerability-detail">
                                                                <strong>"File: "</strong> {v.file_path} " (Line " {v.line_no} ")"<br/>
                                                                <strong>"Recommendation: "</strong> {v.recommendation}
                                                            </div>
                                                        </div>
                                                    }
                                                }/>
                                            </div>
                                        }.into_view()
                                    }}
                                </div>
                            }.into_view()
                        },
                        None => view! { <div>"Failed to load scan report."</div> }.into_view(),
                    })}
                </Suspense>
            </div>
        }
}

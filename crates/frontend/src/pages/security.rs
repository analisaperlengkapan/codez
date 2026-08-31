use leptos::*;
use gloo_net::http::Request;
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
                .send().await.unwrap().json::<SecurityScanReport>().await.ok()
        }
    );

    let on_run_scan = move |_| {
        let o = owner();
        let r = repo_name();
        set_scanning.set(true);
        spawn_local(async move {
            let _ = Request::post(&format!("/api/v1/repos/{}/{}/security/scan", o, r))
                .send().await;
            set_scanning.set(false);
            set_refresh.update(|n| *n += 1);
        });
    };

    view! {
        <div class="security-dashboard" style="padding: 20px; max-width: 900px; margin: 0 auto;">
            <div style="display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid #ccc; padding-bottom: 10px; margin-bottom: 20px;">
                <h2>"🛡️ Security Audit & Vulnerability Scanner"</h2>
                <button
                    style="padding: 8px 16px; background-color: #2da44e; color: white; border: none; border-radius: 6px; cursor: pointer;"
                    on:click=on_run_scan
                    disabled=move || scanning.get()
                >
                    {move || if scanning.get() { "Scanning..." } else { "Run Security Audit" }}
                </button>
            </div>

            <Suspense fallback=move || view! { <div>"Loading security report..."</div> }>
                {move || report.get().map(|rep| match rep {
                    Some(rep) => {
                        let score_color = if rep.score >= 80 { "#2da44e" } else if rep.score >= 50 { "#d97706" } else { "#cf222e" };
                        view! {
                            <div>
                                <div style="display: flex; gap: 20px; align-items: center; background: #f6f8fa; padding: 15px; border-radius: 8px; margin-bottom: 20px;">
                                    <div style=format!("font-size: 32px; font-weight: bold; color: {}; padding: 15px; border: 3px solid {}; border-radius: 50%; width: 60px; height: 60px; display: flex; align-items: center; justify-content: center;", score_color, score_color)>
                                        {rep.score}
                                    </div>
                                    <div>
                                        <h3 style="margin: 0;">"Security Health Score: " {rep.score} "/100"</h3>
                                        <p style="margin: 5px 0 0; color: #57606a;">"Scanned at: " {rep.scanned_at}</p>
                                    </div>
                                </div>

                                <h3>"Vulnerabilities Found (" {rep.vulnerabilities.len()} ")"</h3>
                                {if rep.vulnerabilities.is_empty() {
                                    view! {
                                        <div style="padding: 20px; background: #dafbe1; color: #1a7f37; border-radius: 6px; text-align: center;">
                                            "✅ No vulnerabilities or secret leaks detected in repository!"
                                        </div>
                                    }.into_view()
                                } else {
                                    view! {
                                        <div class="vulnerability-list" style="display: flex; flex-direction: column; gap: 10px;">
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
                                                    <div style=format!("border: 1px solid #d0d7de; border-radius: 6px; padding: 15px; background: {};", badge_bg)>
                                                        <div style="display: flex; justify-content: space-between; align-items: center;">
                                                            <strong style=format!("color: {}; font-size: 16px;", badge_color)>
                                                                "[" {v.severity} "] " {v.title}
                                                            </strong>
                                                            <span style="font-size: 12px; color: #57606a;">{v.id}</span>
                                                        </div>
                                                        <p style="margin: 8px 0; color: #24292f;">{v.description}</p>
                                                        <div style="font-size: 13px; color: #57606a; background: white; padding: 8px; border-radius: 4px; border: 1px dashed #d0d7de;">
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

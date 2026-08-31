use leptos::*;
use gloo_net::http::Request;
use leptos_router::*;
use shared::{ActionWorkflow, WorkflowRun, CreateWorkflowRunOption, UpdateWorkflowRunOption};

#[component]
pub fn ActionsList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());

    let workflows = create_resource(
        move || (owner(), repo_name()),
        |(o, r)| async move {
            Request::get(&format!("/api/v1/repos/{}/{}/actions/workflows", o, r))
                .send().await.unwrap().json::<Vec<ActionWorkflow>>().await.unwrap_or_default()
        }
    );

    view! {
        <div class="actions-list">
            <h3>"Actions Workflows"</h3>
            <ul>
                <Suspense fallback=move || view! { <li>"Loading workflows..."</li> }>
                    {move || workflows.get().map(|list| view! {
                        <For each=move || list.clone() key=|w| w.id children=move |w| {
                            let href = format!("/repos/{}/{}/actions/workflows/{}", owner(), repo_name(), w.id);
                            view! {
                                <li>
                                    <a href=href><strong>{w.name}</strong></a>
                                    " - " {w.status}
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
pub fn WorkflowRunsList() -> impl IntoView {
    let params = use_params_map();
    let owner = move || params.with(|params| params.get("owner").cloned().unwrap_or_default());
    let repo_name = move || params.with(|params| params.get("repo").cloned().unwrap_or_default());
    let workflow_id = move || params.with(|params| params.get("id").cloned().unwrap_or_default().parse::<u64>().unwrap_or_default());

    let (refresh, set_refresh) = create_signal(0);

    let runs = create_resource(
        move || (owner(), repo_name(), workflow_id(), refresh.get()),
        |(o, r, id, _)| async move {
            Request::get(&format!("/api/v1/repos/{}/{}/actions/workflows/{}/runs", o, r, id))
                .send().await.unwrap().json::<Vec<WorkflowRun>>().await.unwrap_or_default()
        }
    );

    let on_run_workflow = move |_| {
        let o = owner();
        let r = repo_name();
        let id = workflow_id();
        let payload = CreateWorkflowRunOption {
            workflow_id: id,
            ref_name: "main".to_string(), // hardcoded for MVP
        };
        spawn_local(async move {
            let _ = Request::post(&format!("/api/v1/repos/{}/{}/actions/workflows/{}/runs", o, r, id))
                .json(&payload).unwrap().send().await;
            set_refresh.update(|n| *n += 1);
        });
    };

    let on_rerun_workflow = move |run_id: u64| {
        let o = owner();
        let r = repo_name();
        spawn_local(async move {
            let _ = Request::post(&format!("/api/v1/repos/{}/{}/actions/runs/{}/rerun", o, r, run_id))
                .send().await;
            set_refresh.update(|n| *n += 1);
        });
    };

    let on_cancel_run = move |run_id: u64| {
        let o = owner();
        let r = repo_name();
        let payload = UpdateWorkflowRunOption { status: "cancelled".to_string() };
        spawn_local(async move {
            let _ = Request::patch(&format!("/api/v1/repos/{}/{}/actions/runs/{}", o, r, run_id))
                .json(&payload).unwrap().send().await;
            set_refresh.update(|n| *n += 1);
        });
    };

    let on_delete_run = move |run_id: u64| {
        let o = owner();
        let r = repo_name();
        spawn_local(async move {
            let _ = Request::delete(&format!("/api/v1/repos/{}/{}/actions/runs/{}", o, r, run_id))
                .send().await;
            set_refresh.update(|n| *n += 1);
        });
    };

    view! {
        <div class="workflow-runs">
            <div class="header" style="display: flex; justify-content: space-between;">
                <h3>"Workflow Runs"</h3>
                <button class="run-workflow-btn" on:click=on_run_workflow>"Run Workflow"</button>
            </div>
            <ul class="runs-list">
                <Suspense fallback=move || view! { <li>"Loading runs..."</li> }>
                    {move || runs.get().map(|list| view! {
                        <For each=move || list.clone() key=|r| r.id children=move |r| {
                            let status = r.status.clone();
                            let is_active = status == "queued" || status == "in_progress";
                            let run_id = r.id;
                            let step_logs = r.step_logs.clone();
                            let on_cancel = { move |_| on_cancel_run(run_id) };
                            let on_delete = { move |_| on_delete_run(run_id) };
                            let on_rerun = { move |_| on_rerun_workflow(run_id) };
                            view! {
                                <li class="run-item" style="border: 1px solid #e1e4e8; padding: 10px; margin-bottom: 10px; border-radius: 6px;">
                                    <div style="display: flex; justify-content: space-between; align-items: center;">
                                        <span>"Run #" {r.id} " - " <span class="run-status" style="font-weight: bold;">{r.status}</span> " (" {r.created_at} ")"</span>
                                        <div style="display: flex; gap: 5px;">
                                            <button class="rerun-btn" style="padding: 4px 8px; background: #0969da; color: white; border: none; border-radius: 4px;" on:click=on_rerun>"Re-run"</button>
                                            {if is_active {
                                                view! { <button class="cancel-run-btn" on:click=on_cancel>"Cancel"</button> }.into_view()
                                            } else {
                                                view! { <button class="delete-run-btn" on:click=on_delete>"Delete"</button> }.into_view()
                                            }}
                                        </div>
                                    </div>
                                    {if !step_logs.is_empty() {
                                        view! {
                                            <div style="margin-top: 10px; background: #0d1117; color: #c9d1d9; padding: 10px; border-radius: 4px; font-family: monospace; font-size: 12px;">
                                                <For each=move || step_logs.clone() key=|s| s.name.clone() children=move |s| {
                                                    let logs = s.logs.clone();
                                                    view! {
                                                        <div style="margin-bottom: 8px;">
                                                            <div style="color: #58a6ff; font-weight: bold;">"▶ " {s.name} " (" {s.status} ")"</div>
                                                            <For each=move || logs.clone() key=|l| l.clone() children=move |l| {
                                                                view! { <div style="padding-left: 15px; color: #8b949e;">{l}</div> }
                                                            }/>
                                                        </div>
                                                    }
                                                }/>
                                            </div>
                                        }.into_view()
                                    } else {
                                        view! {}.into_view()
                                    }}
                                </li>
                            }
                        }/>
                    })}
                </Suspense>
            </ul>
        </div>
    }
}

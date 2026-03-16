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
                                    let on_cancel = { move |_| on_cancel_run(run_id) };
                                    let on_delete = { move |_| on_delete_run(run_id) };
                            view! {
                                <li class="run-item" style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 5px;">
                                    <span>"Run #" {r.id} " - " <span class="run-status">{r.status}</span> " (" {r.created_at} ")"</span>
                                    <div>
                                        {if is_active {
                                            view! { <button class="cancel-run-btn" on:click=on_cancel>"Cancel"</button> }.into_view()
                                        } else {
                                            view! { <button class="delete-run-btn" on:click=on_delete>"Delete"</button> }.into_view()
                                        }}
                                    </div>
                                </li>
                            }
                        }/>
                    })}
                </Suspense>
            </ul>
        </div>
    }
}

CREATE TABLE IF NOT EXISTS ci_pipeline_executions (
    id UUID PRIMARY KEY,
    provider VARCHAR(50) NOT NULL,
    repo VARCHAR(255) NOT NULL,
    git_ref VARCHAR(255) NOT NULL,
    commit_sha VARCHAR(255) NOT NULL,
    pipeline_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS ci_job_executions (
    id UUID PRIMARY KEY,
    pipeline_id UUID NOT NULL,
    provider VARCHAR(50) NOT NULL,
    repo VARCHAR(255) NOT NULL,
    git_ref VARCHAR(255) NOT NULL,
    commit_sha VARCHAR(255) NOT NULL,
    job_id UUID NOT NULL,
    job_name VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL,
    started_at TIMESTAMPTZ,
    finished_at TIMESTAMPTZ,
    duration_seconds BIGINT,
    log_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_ci_pipeline_executions_pipeline_id ON ci_pipeline_executions(pipeline_id);
CREATE INDEX IF NOT EXISTS idx_ci_job_executions_pipeline_id ON ci_job_executions(pipeline_id);

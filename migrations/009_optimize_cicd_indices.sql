-- Add indices for CI/CD job lookups by status and repo
CREATE INDEX IF NOT EXISTS idx_ci_job_executions_status ON ci_job_executions(status);
CREATE INDEX IF NOT EXISTS idx_ci_job_executions_repo ON ci_job_executions(repo);

-- Add composite index for efficient job history retrieval by repo
CREATE INDEX IF NOT EXISTS idx_ci_job_executions_repo_created_at ON ci_job_executions(repo, created_at DESC);

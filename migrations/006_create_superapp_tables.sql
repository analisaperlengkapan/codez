CREATE TABLE super_apps (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    version VARCHAR(50) NOT NULL,
    description TEXT,
    config JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE super_app_modules (
    id UUID PRIMARY KEY,
    super_app_id UUID NOT NULL REFERENCES super_apps(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    version VARCHAR(50) NOT NULL,
    remote_entry VARCHAR(255) NOT NULL,
    scope VARCHAR(255) NOT NULL,
    dependencies JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS micro_frontends (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    version VARCHAR(50) NOT NULL,
    remote_entry VARCHAR(255) NOT NULL,
    scope VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS mfe_dependencies (
    mfe_id UUID NOT NULL REFERENCES micro_frontends(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    version VARCHAR(50) NOT NULL,
    PRIMARY KEY (mfe_id, name)
);

CREATE TABLE IF NOT EXISTS mfe_shared_dependencies (
    mfe_id UUID NOT NULL REFERENCES micro_frontends(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    version VARCHAR(50) NOT NULL,
    singleton BOOLEAN DEFAULT FALSE,
    strict_version BOOLEAN DEFAULT FALSE,
    PRIMARY KEY (mfe_id, name)
);

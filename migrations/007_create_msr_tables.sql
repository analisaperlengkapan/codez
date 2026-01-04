-- Microservices table
CREATE TABLE IF NOT EXISTS microservices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    version VARCHAR(50) NOT NULL,
    host VARCHAR(255) NOT NULL,
    port INTEGER NOT NULL,
    protocol VARCHAR(20) NOT NULL DEFAULT 'http',
    status VARCHAR(50) NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    tags TEXT[] NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for faster lookups by name and version
CREATE INDEX IF NOT EXISTS idx_microservices_name_version ON microservices(name, version);
-- Index for finding healthy services
CREATE INDEX IF NOT EXISTS idx_microservices_status ON microservices(status);

-- Add indices for faster lookups in micro_frontends table
CREATE INDEX IF NOT EXISTS idx_micro_frontends_name ON micro_frontends(name);
CREATE INDEX IF NOT EXISTS idx_micro_frontends_status ON micro_frontends(status);

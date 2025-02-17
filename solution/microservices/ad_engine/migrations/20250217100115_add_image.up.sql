-- Add up migration script here

CREATE TABLE IF NOT EXISTS campaigns_images (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    data BYTEA NOT NULL,
    mime_type VARCHAR(20) NOT NULL CHECK (mime_type IN ('image/jpeg', 'image/png')),
    file_name VARCHAR(255) NOT NULL,
    file_size BIGINT NOT NULL CHECK (file_size <= 52428800), -- 50MB
    campaign_id UUID NOT NULL REFERENCES campaigns(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX ON campaigns_images (campaign_id);

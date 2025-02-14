-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS likes_clients (
    campaign_id UUID NOT NULL REFERENCES campaigns(id),
    client_id UUID NOT NULL REFERENCES clients(id),
    PRIMARY KEY (campaign_id, client_id)
);

CREATE TABLE IF NOT EXISTS views_clients (
    campaign_id UUID NOT NULL REFERENCES campaigns(id),
    client_id UUID NOT NULL REFERENCES clients(id),
    PRIMARY KEY (campaign_id, client_id)
);

CREATE INDEX idx_likes_clients_campaign_id ON likes_clients (campaign_id);

CREATE INDEX idx_views_clients_campaign_id ON views_clients (campaign_id);

-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS campaigns (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(), 
    advertiser_id UUID NOT NULL REFERENCES advertisers(id),
    impressions_limit INT NOT NULL CHECK (impressions_limit >= 0), 
    clicks_limit INT NOT NULL CHECK (clicks_limit >= 0),
    cost_per_impressions NUMERIC(10, 2) NOT NULL CHECK (cost_per_impressions >= 0), 
    cost_per_clicks NUMERIC(10, 2) NOT NULL CHECK (cost_per_clicks >= 0), 
    ad_title VARCHAR(255) NOT NULL, 
    ad_text TEXT NOT NULL,
    start_date INT NOT NULL CHECK (start_date >= 0),
    end_date INT NOT NULL CHECK (end_date >= 0),
    targeting JSONB NOT NULL, 
    created_at INT NOT NULL,
    updated_at INT NOT NULL,

    CONSTRAINT valid_dates CHECK (start_date <= end_date)
);

CREATE INDEX IF NOT EXISTS campaigns_advertiser_id_idx ON campaigns (advertiser_id);

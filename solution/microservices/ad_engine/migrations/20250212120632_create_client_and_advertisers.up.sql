-- Add migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS clients (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    login VARCHAR(64) UNIQUE NOT NULL,
    location VARCHAR(128) NOT NULL,
    gender VARCHAR(6) NOT NULL,
    age INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
COMMENT ON COLUMN clients.gender IS 'Possible value: FEMALE, MALE';

CREATE TABLE IF NOT EXISTS advertisers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(64) UNIQUE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS ml_scores (
    PRIMARY KEY (client_id, advertiser_id),
    client_id UUID NOT NULL REFERENCES clients(id),
    advertiser_id UUID NOT NULL REFERENCES advertisers(id),
    score FLOAT NOT NULL
);

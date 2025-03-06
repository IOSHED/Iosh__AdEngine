-- Add down migration script here

DROP INDEX IF EXISTS campaigns_advertiser_id_idx;

DROP TABLE IF EXISTS campaigns CASCADE;

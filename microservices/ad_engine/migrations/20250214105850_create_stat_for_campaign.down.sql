-- Add down migration script here

DROP INDEX IF EXISTS idx_views_clients_campaign_id;
DROP INDEX IF EXISTS idx_likes_clients_campaign_id;

DROP TABLE IF EXISTS views_clients CASCADE;
DROP TABLE IF EXISTS likes_clients CASCADE;
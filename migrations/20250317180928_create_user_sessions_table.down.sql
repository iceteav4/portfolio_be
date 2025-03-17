-- Add down migration script here
DROP INDEX idx_user_sessions_user_id;
DROP TABLE user_sessions;
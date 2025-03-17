-- Add down migration script here
DROP INDEX IF EXISTS users_email_idx;
DROP TABLE IF EXISTS users;

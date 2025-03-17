-- Add up migration script here

CREATE TABLE user_sessions (
    session_id BIGINT PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ
);

-- Add index for faster user lookups
CREATE INDEX idx_user_sessions_user_id ON user_sessions(user_id);

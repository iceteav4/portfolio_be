-- Add up migration script here
CREATE TABLE users (
    id BIGINT PRIMARY KEY,
    status VARCHAR(20) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    phone_number VARCHAR(20) UNIQUE,
    hashed_password VARCHAR(255),
    name VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX users_email_idx ON users (email);

CREATE TABLE user_sessions (
    session_id BIGINT PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users (id),
    is_active BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_user_sessions_user_id ON user_sessions (user_id);

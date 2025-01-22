-- Add migration script here
-- Create Users Table
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    uid INTEGER NOT NULL,
    openid VARCHAR(255) NOT NULL,
    session_key VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create index on uid and openid for faster lookups
CREATE INDEX idx_users_uid ON users(uid);
CREATE INDEX idx_users_openid ON users(openid);

---- Down migration
DROP TABLE IF EXISTS users;

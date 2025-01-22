-- Add down migration script here

-- Drop indexes first
DROP INDEX IF EXISTS idx_users_uid;
DROP INDEX IF EXISTS idx_users_openid;

-- Drop the users table
DROP TABLE IF EXISTS users;

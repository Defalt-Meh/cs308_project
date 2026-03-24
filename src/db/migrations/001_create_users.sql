-- Migration 001: Create users table
--
-- Covers:
--   Req 10 — Three roles: customer, sales_manager, product_manager
--   Req 13 — Customer properties: ID, name, tax_id, email, home_address, password
--   Req 16 — password_hash stores argon2 output (never plaintext)
--
-- The role is stored as a PostgreSQL enum for type safety. Trying to
-- insert an invalid role will fail at the DB level, not just in Rust.

CREATE TYPE user_role AS ENUM ('customer', 'sales_manager', 'product_manager');

CREATE TABLE IF NOT EXISTS users (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email       VARCHAR(255) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    name        VARCHAR(255) NOT NULL,
    tax_id      VARCHAR(50),
    home_address TEXT,
    role        user_role NOT NULL DEFAULT 'customer',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index on email for fast login lookups.
CREATE INDEX IF NOT EXISTS idx_users_email ON users (email);

-- Index on role for admin queries that filter by manager type.
CREATE INDEX IF NOT EXISTS idx_users_role ON users (role);
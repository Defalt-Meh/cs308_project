//! # User Model
//!
//! Defines the `User` database struct and associated DTOs for
//! authentication and registration flows.
//!
//! ## Security (Requirement 16)
//! - `password_hash` is never exposed outside the services layer.
//! - `UserResponse` is the only struct passed to templates.
//! - The `UserRole` enum mirrors the Postgres `user_role` type.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── Role Enum ────────────────────────────────────────────────

/// Maps to the `user_role` Postgres enum.
/// `sqlx::Type` lets sqlx read/write this directly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
pub enum UserRole {
    Customer,
    SalesManager,
    ProductManager,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Customer       => write!(f, "customer"),
            Self::SalesManager   => write!(f, "sales_manager"),
            Self::ProductManager => write!(f, "product_manager"),
        }
    }
}

// ── Database Row ─────────────────────────────────────────────

/// Full user record as stored in Postgres. Only used internally
/// by services — never sent to templates or clients directly.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub tax_id: Option<String>,
    pub home_address: Option<String>,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ── Request DTOs ─────────────────────────────────────────────

/// Submitted by the login form. Both fields are required.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Submitted by the registration form. Only customers can
/// self-register; manager accounts are created by admins.
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
    pub tax_id: Option<String>,
    pub home_address: Option<String>,
}

// ── Template-safe Response ───────────────────────────────────

/// Safe subset of `User` that can be passed into Tera templates
/// and serialized for HTMX responses. Note: no `password_hash`.
#[derive(Debug, Clone, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub tax_id: Option<String>,
    pub home_address: Option<String>,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            email: u.email,
            name: u.name,
            tax_id: u.tax_id,
            home_address: u.home_address,
            role: u.role,
            created_at: u.created_at,
        }
    }
}

// ── JWT Claims ───────────────────────────────────────────────

/// Payload embedded in the JWT token. Kept minimal to reduce
/// token size — fetch full user data from DB when needed.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// User UUID as string.
    pub sub: String,
    /// User role for quick middleware checks without a DB query.
    pub role: String,
    /// Expiration timestamp (seconds since epoch).
    pub exp: usize,
    /// Issued-at timestamp.
    pub iat: usize,
}
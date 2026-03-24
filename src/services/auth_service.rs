//! # Authentication Service
//!
//! Handles the core auth flows: registration, login, and JWT
//! token management. All password operations delegate to
//! `encryption_service` so the hashing algorithm can be swapped
//! without touching auth logic.
//!
//! ## Security (Requirement 16)
//! - Passwords are hashed with argon2id before storage.
//! - JWT tokens are signed with HS256 using the server's secret.
//! - Tokens carry minimal claims (user ID + role) to limit
//!   exposure if a token is leaked.
//! - Failed logins return a generic "invalid credentials" message
//!   to prevent email enumeration.

use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::AppConfig;
use crate::errors::{AppError, Result};
use crate::models::user::{Claims, LoginRequest, RegisterRequest, User, UserRole};
use crate::services::encryption_service;

// ── Registration ─────────────────────────────────────────────

/// Registers a new customer account. Manager accounts cannot be
/// created through this path — they must be seeded or created by
/// an existing admin.
///
/// Returns the newly created user's UUID.
///
/// # Errors
/// - `AppError::Conflict` if the email is already registered.
/// - `AppError::Validation` if required fields are empty.
pub async fn register(pool: &PgPool, req: &RegisterRequest) -> Result<Uuid> {
    // Validate required fields.
    if req.email.trim().is_empty() || req.password.is_empty() || req.name.trim().is_empty() {
        return Err(AppError::Validation(
            "Email, password, and name are required".into(),
        ));
    }

    // Check for duplicate email.
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)"
    )
    .bind(&req.email)
    .fetch_one(pool)
    .await?;

    if exists {
        return Err(AppError::Conflict(
            "An account with this email already exists".into(),
        ));
    }

    // Hash the password (argon2id).
    let password_hash = encryption_service::hash_password(&req.password)?;

    // Insert the new user.
    let id = sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO users (email, password_hash, name, tax_id, home_address, role)
        VALUES ($1, $2, $3, $4, $5, 'customer')
        RETURNING id
        "#,
    )
    .bind(&req.email)
    .bind(&password_hash)
    .bind(&req.name)
    .bind(&req.tax_id)
    .bind(&req.home_address)
    .fetch_one(pool)
    .await?;

    tracing::info!(user_id = %id, email = %req.email, "New user registered");
    Ok(id)
}

// ── Login ────────────────────────────────────────────────────

/// Validates credentials and returns a signed JWT token string.
///
/// # Errors
/// - `AppError::InvalidCredentials` on wrong email or password.
///   The error is intentionally vague to prevent enumeration.
pub async fn login(pool: &PgPool, config: &AppConfig, req: &LoginRequest) -> Result<String> {
    // Look up the user by email. Using `optional()` so a missing
    // row doesn't surface as a database error.
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&req.email)
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::InvalidCredentials)?;

    // Verify the password against the stored hash.
    if !encryption_service::verify_password(&req.password, &user.password_hash)? {
        return Err(AppError::InvalidCredentials);
    }

    // Build and sign a JWT.
    let token = create_token(&user.id, &user.role, config)?;

    tracing::info!(user_id = %user.id, role = %user.role, "User logged in");
    Ok(token)
}

// ── Token Management ─────────────────────────────────────────

/// Creates a signed JWT with the user's ID and role embedded
/// in the claims. The token expires after `config.jwt_expiry_hours`.
fn create_token(user_id: &Uuid, role: &UserRole, config: &AppConfig) -> Result<String> {
    let now = Utc::now();
    let expiry = now + chrono::Duration::hours(config.jwt_expiry_hours);

    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_string(),
        iat: now.timestamp() as usize,
        exp: expiry.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("JWT encoding failed: {e}")))?;

    Ok(token)
}

/// Decodes and validates a JWT token string. Returns the embedded
/// claims if the signature is valid and the token hasn't expired.
pub fn validate_token(token: &str, config: &AppConfig) -> Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized)?;

    Ok(token_data.claims)
}

// ── Lookup Helpers ───────────────────────────────────────────

/// Fetches a user by ID. Used by middleware after extracting the
/// user ID from a validated JWT.
pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<User> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await?
        .ok_or(AppError::NotFound("User not found".into()))
}
//! # Domain Models
//!
//! Each sub-module defines:
//! 1. A **database struct** — maps 1:1 to a table row (`sqlx::FromRow`).
//! 2. **Request DTOs** — what the handler receives from forms / JSON.
//! 3. **Response DTOs** — what gets passed into Tera templates. These
//!    intentionally omit sensitive fields like `password_hash`.
//!
//! Keeping DB structs and DTOs separate prevents accidental leakage
//! of internal fields (password hashes, internal IDs) into templates.

pub mod user;
pub mod product;
pub mod category;
pub mod cart;
pub mod cart_item;
pub mod order;
pub mod order_item;
pub mod review;
pub mod wishlist;
pub mod invoice;
pub mod delivery;
pub mod refund;
pub mod discount;
pub mod notification;
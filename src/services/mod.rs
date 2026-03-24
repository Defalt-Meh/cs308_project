//! # Services Layer
//!
//! Business logic lives here — not in route handlers, not in models.
//! Each service receives a `&PgPool` (or relevant deps) and returns
//! domain types or `AppError`. Handlers are thin wrappers that parse
//! the request, call a service, and render a template with the result.
//!
//! ## Why a separate layer?
//! - **Testability**: services can be unit-tested with a test database
//!   and no HTTP machinery.
//! - **Reuse**: the same `order_service::place_order()` can be called
//!   from both a page handler and an HTMX partial handler.
//! - **Clarity**: route files stay short; business rules are obvious.

pub mod auth_service;
pub mod encryption_service;
pub mod product_service;
pub mod category_service;
pub mod cart_service;
pub mod order_service;
pub mod review_service;
pub mod wishlist_service;
pub mod payment_service;
pub mod refund_service;
pub mod pdf_service;
pub mod email_service;
pub mod notification_service;
pub mod delivery_service;
pub mod discount_service;
pub mod analytics_service;
pub mod stock_service;
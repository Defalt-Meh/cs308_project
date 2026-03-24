# Architecture Overview

## Tech Stack

```
Browser ──HTMX──▸ Axum (Rust) ──sqlx──▸ PostgreSQL
                     │
                     ├─ Tera templates (server-rendered HTML)
                     ├─ tower-http (static files, CORS, gzip)
                     └─ argon2id + JWT (auth)
```

All pages are server-rendered. HTMX handles dynamic interactions
(search, add-to-cart, form submissions) by swapping HTML fragments
returned from the server. No JavaScript framework — one `<script>`
tag for HTMX, one `app.js` for toast management and keyboard shortcuts.

## Request Flow

```
Request → Axum Router
  → Auth Middleware (extract JWT from cookie, set AuthUser)
  → Role Guard (admin routes only)
  → Route Handler (parse request, call service)
  → Service Layer (business logic, DB queries)
  → Tera Template (render HTML with context data)
  → Response (full page or HTMX partial fragment)
```

## File Tree

Legend:
- `[✓]` — Implemented (has real code)
- `[ ]` — Stub (empty file, ready to implement)

```
cs308_project/
│
├── [✓] Cargo.toml                          # Rust dependencies: axum, sqlx, tera, argon2, jwt, lettre, genpdf
├── [✓] Dockerfile                          # Multi-stage: rust:1.78 builder → debian:slim runtime (~80MB)
├── [✓] docker-compose.yml                  # Postgres 16 + backend, one-command startup
├── [✓] README.md                           # Project overview, quick start, sprint schedule
├── [✓] .gitignore                          # Rust target/, .env, IDE, OS files
├── [✓] .env.example                        # All config vars documented with defaults
│
├── .github/
│   └── workflows/
│       └── [ ] ci.yml                      # GitHub Actions: cargo build + test + clippy
│
├── docs/
│   ├── [✓] architecture.md                 # This file — full repo map and design decisions
│   ├── [ ] api.md                          # REST endpoint reference table
│   └── [ ] database_schema.md              # ER diagram, table descriptions
│
│
│   ═══════════════════════════════════════
│   SOURCE — Rust (Axum)
│   ═══════════════════════════════════════
│
├── src/
│   ├── [✓] main.rs                         # Entry point: logging → config → DB pool → Tera → router → serve
│   ├── [✓] config.rs                       # AppConfig: reads env vars (DB, JWT, SMTP)
│   ├── [✓] errors.rs                       # AppError enum → auto HTTP status + HTML toast fragment
│   ├── [✓] lib.rs                          # Module re-exports for integration tests
│   │
│   ├── db/
│   │   ├── [✓] mod.rs                      # Module declaration
│   │   ├── [✓] pool.rs                     # PgPool: 20 max conn, 5 warm, 5s timeout (Req 17 concurrency)
│   │   │
│   │   └── migrations/
│   │       ├── [✓] 001_create_users.sql    # users table + user_role enum (Req 10, 13, 16)
│   │       ├── [ ] 002_create_categories.sql
│   │       ├── [ ] 003_create_products.sql  # Req 9: ID, name, model, serial, desc, qty, price, warranty, distributor
│   │       ├── [ ] 004_create_carts.sql
│   │       ├── [ ] 005_create_orders.sql    # Req 3: status enum (processing, in_transit, delivered)
│   │       ├── [ ] 006_create_order_items.sql
│   │       ├── [ ] 007_create_reviews.sql   # Req 5: rating 1-10, comment text, approved boolean
│   │       ├── [ ] 008_create_wishlists.sql # Req 11: for discount notifications
│   │       ├── [ ] 009_create_invoices.sql  # Req 4, 11: PDF generation + date-range queries
│   │       ├── [ ] 010_create_deliveries.sql # Req 12: delivery_id, customer_id, product_id, qty, price, address, completed
│   │       ├── [ ] 011_create_refunds.sql   # Req 15: 30-day window, discount-price preservation
│   │       ├── [ ] 012_create_discounts.sql # Req 11: rate, product selection, wishlist notifications
│   │       └── [ ] 013_create_notifications.sql
│   │
│   ├── models/
│   │   ├── [✓] mod.rs                      # Declares all 15 model sub-modules
│   │   ├── [✓] user.rs                     # User, UserRole, LoginRequest, RegisterRequest, UserResponse, Claims
│   │   ├── [ ] product.rs                  # Product, ProductResponse, CreateProductRequest
│   │   ├── [ ] category.rs                 # Category, CategoryResponse
│   │   ├── [ ] cart.rs                     # Cart (session-based for anonymous, user-bound for logged-in)
│   │   ├── [ ] cart_item.rs                # CartItem with product reference and quantity
│   │   ├── [ ] order.rs                    # Order with status enum, OrderResponse
│   │   ├── [ ] order_item.rs               # OrderItem — line items within an order
│   │   ├── [ ] review.rs                   # Review with rating, comment, approved flag
│   │   ├── [ ] wishlist.rs                 # Wishlist entry linking user ↔ product
│   │   ├── [ ] invoice.rs                  # Invoice with PDF path, total, date
│   │   ├── [ ] delivery.rs                 # Delivery list record (Req 12)
│   │   ├── [ ] refund.rs                   # Refund request with status, original purchase price
│   │   ├── [ ] discount.rs                 # Discount campaign: rate, product set, date range
│   │   └── [ ] notification.rs             # User notifications (discount alerts, order updates)
│   │
│   ├── routes/
│   │   ├── [✓] mod.rs                      # build_router(): assembles all route groups + middleware layers
│   │   ├── [✓] auth.rs                     # GET+POST /auth/login, /auth/register, POST /auth/logout
│   │   ├── [✓] pages.rs                    # GET / (home), fallback 404 — both auth-aware
│   │   ├── [ ] products.rs                 # GET /products, /products/:id, /products/search (HTMX partial)
│   │   ├── [ ] categories.rs               # GET /categories, /categories/:id
│   │   ├── [ ] cart.rs                     # GET /cart, POST /cart/add, /cart/remove, /cart/update
│   │   ├── [ ] orders.rs                   # GET /orders, /orders/:id, POST /orders/place, /orders/:id/cancel
│   │   ├── [ ] reviews.rs                  # POST /reviews, GET /reviews/product/:id (HTMX partial)
│   │   ├── [ ] wishlist.rs                 # GET /wishlist, POST /wishlist/add, /wishlist/remove
│   │   ├── [ ] payments.rs                 # POST /payments/process (mock bank), GET /payments/result
│   │   ├── [ ] refunds.rs                  # GET /refunds/request/:order_item_id, POST /refunds/submit
│   │   │
│   │   └── admin/
│   │       ├── [ ] mod.rs                  # Admin router: mounts sales + product_mgr with role guards
│   │       ├── [ ] sales.rs                # Sales manager dashboard
│   │       ├── [ ] product_mgr.rs          # Product manager dashboard
│   │       ├── [ ] deliveries.rs           # GET /admin/deliveries, POST /admin/deliveries/:id/complete
│   │       ├── [ ] comments.rs             # GET /admin/comments/pending, POST /admin/comments/:id/approve|reject
│   │       ├── [ ] analytics.rs            # GET /admin/analytics (revenue/profit chart data)
│   │       ├── [ ] invoices.rs             # GET /admin/invoices?from=&to= (date-range, PDF export)
│   │       ├── [ ] categories.rs           # POST /admin/categories/add, /admin/categories/:id/delete
│   │       ├── [ ] discounts.rs            # POST /admin/discounts/apply (triggers wishlist notifications)
│   │       ├── [ ] stock.rs                # GET /admin/stock, POST /admin/stock/:id/update
│   │       └── [ ] refund_mgr.rs           # GET /admin/refunds/pending, POST /admin/refunds/:id/approve
│   │
│   ├── middleware/
│   │   ├── [✓] mod.rs                      # Declares auth, role_guard, cors
│   │   ├── [✓] auth.rs                     # Reads token cookie → validates JWT → injects Option<AuthUser>
│   │   ├── [✓] role_guard.rs               # role_guard() + require_sales_manager/require_product_manager
│   │   └── [✓] cors.rs                     # CorsLayer builder (permissive in dev, lockable for prod)
│   │
│   └── services/
│       ├── [✓] mod.rs                      # Declares all 17 service sub-modules
│       ├── [✓] auth_service.rs             # register(), login(), validate_token(), get_user_by_id()
│       ├── [✓] encryption_service.rs       # hash_password(), verify_password() — argon2id + unit tests
│       ├── [ ] product_service.rs          # CRUD, search by name/desc, sort by price/popularity
│       ├── [ ] category_service.rs         # Add/remove categories
│       ├── [ ] cart_service.rs             # Add/remove/update items, stock check before add
│       ├── [ ] order_service.rs            # Place order (decrement stock), cancel, status transitions
│       ├── [ ] review_service.rs           # Submit review, list by product (approved only)
│       ├── [ ] wishlist_service.rs         # Add/remove, query by user, query by product (for notifications)
│       ├── [ ] payment_service.rs          # Mock bank verification, credit card validation
│       ├── [ ] refund_service.rs           # 30-day check, discount-price preservation, stock re-add
│       ├── [ ] pdf_service.rs              # Invoice PDF generation with genpdf
│       ├── [ ] email_service.rs            # Send invoice PDF via lettre (log-only in dev)
│       ├── [ ] notification_service.rs     # Discount alerts to wishlist users
│       ├── [ ] delivery_service.rs         # Delivery list CRUD, mark as completed
│       ├── [ ] discount_service.rs         # Apply discount to products, trigger notifications
│       ├── [ ] analytics_service.rs        # Revenue/profit calculation between dates
│       └── [ ] stock_service.rs            # Stock queries, row-level locking for concurrent decrements
│
│
│   ═══════════════════════════════════════
│   TEMPLATES — Tera (HTML)
│   ═══════════════════════════════════════
│
├── templates/
│   │
│   ├── [✓] base.html                       # Master layout: head, fonts, CSS, nav, footer, HTMX, toast container
│   ├── [ ] base_admin.html                 # Admin layout: sidebar nav, role-specific links
│   │
│   ├── components/
│   │   ├── [✓] navbar.html                 # Auth-aware nav: brand, links, cart, user info, logout (HTMX)
│   │   ├── [✓] footer.html                 # Terminal-style footer with status dot
│   │   ├── [ ] sidebar.html                # Admin sidebar navigation
│   │   ├── [ ] product_card.html           # Product card: image, name, price, stock, add-to-cart
│   │   ├── [ ] cart_item.html              # Single cart line: product, qty controls, subtotal
│   │   ├── [ ] cart_drawer.html            # Slide-out cart summary
│   │   ├── [ ] review_card.html            # Single review: author, stars, comment, date
│   │   ├── [ ] review_form.html            # Star rating selector + comment textarea
│   │   ├── [ ] star_rating.html            # Reusable 1-5 or 1-10 star display
│   │   ├── [ ] search_bar.html             # Standalone search (used outside hero)
│   │   ├── [ ] sort_dropdown.html          # Price/popularity sort selector
│   │   ├── [ ] pagination.html             # Page number buttons
│   │   ├── [ ] toast.html                  # Toast notification template
│   │   ├── [ ] modal.html                  # Generic modal wrapper
│   │   ├── [ ] stock_indicator.html        # Green/yellow/red stock status
│   │   ├── [ ] order_status_badge.html     # Processing / In-transit / Delivered badge
│   │   ├── [ ] order_timeline.html         # Visual timeline of order status changes
│   │   └── [ ] invoice_view.html           # Invoice detail display
│   │
│   ├── pages/
│   │   ├── [✓] home.html                   # Hero + triangle + search + featured products grid
│   │   ├── [✓] login.html                  # Auth card: HTMX form POST, error swap, grid bg
│   │   ├── [✓] register.html               # Auth card: name, email, password, tax_id, address
│   │   ├── [✓] not_found.html              # 404: giant digits, diagnostic code block
│   │   ├── [ ] product_list.html           # Browse all products with search + sort + pagination
│   │   ├── [ ] product_detail.html         # Single product: images, specs, stock, reviews, add-to-cart
│   │   ├── [ ] cart.html                   # Full cart page: items, totals, proceed to checkout
│   │   ├── [ ] checkout.html               # Address, credit card form, order summary
│   │   ├── [ ] payment_result.html         # Success/failure screen after mock payment
│   │   ├── [ ] order_history.html          # List of user's orders with status badges
│   │   ├── [ ] order_detail.html           # Single order: items, timeline, invoice link
│   │   ├── [ ] wishlist.html               # User's wishlist with remove buttons
│   │   ├── [ ] profile.html                # User profile: name, email, address, password change
│   │   └── [ ] refund_request.html         # Select order item → submit return reason
│   │
│   ├── admin/
│   │   ├── [ ] dashboard.html              # Overview: pending orders, low stock alerts, stats
│   │   ├── [ ] manage_products.html        # Product CRUD table with inline editing
│   │   ├── [ ] manage_categories.html      # Category list with add/remove
│   │   ├── [ ] manage_orders.html          # All orders, filter by status, update status
│   │   ├── [ ] manage_deliveries.html      # Delivery list with addresses, mark as completed
│   │   ├── [ ] review_moderation.html      # Pending comments: approve/reject buttons
│   │   ├── [ ] pricing_discounts.html      # Select products → set discount rate → apply
│   │   ├── [ ] invoice_manager.html        # Date range picker → invoice table → PDF export
│   │   ├── [ ] revenue_analytics.html      # Date range → revenue/profit chart
│   │   ├── [ ] refund_manager.html         # Pending refunds: approve → restock + refund
│   │   └── [ ] stock_manager.html          # Stock levels table with quantity editing
│   │
│   └── partials/                            # HTMX fragments (swapped into pages, not full documents)
│       ├── [ ] product_grid.html            # Grid of product cards (search results)
│       ├── [ ] cart_summary.html            # Cart totals (refreshed on add/remove)
│       ├── [ ] search_results.html          # Search result list
│       ├── [ ] review_list.html             # Reviews for a product (paginated)
│       ├── [ ] order_list.html              # Order history rows
│       ├── [ ] delivery_table.html          # Delivery table rows
│       ├── [ ] invoice_table.html           # Invoice table rows
│       ├── [ ] stock_table.html             # Stock table rows
│       ├── [ ] comment_list.html            # Pending comments list
│       ├── [ ] refund_list.html             # Pending refund requests
│       ├── [ ] discount_form.html           # Discount application form
│       ├── [ ] category_form.html           # Category add form
│       ├── [ ] notification_list.html       # User notification dropdown
│       ├── [ ] revenue_chart.html           # Chart.js or SVG chart fragment
│       └── [ ] wishlist_items.html          # Wishlist product cards
│
│
│   ═══════════════════════════════════════
│   STATIC ASSETS
│   ═══════════════════════════════════════
│
├── static/
│   ├── css/
│   │   ├── [✓] theme.css                   # 70+ CSS variables: colors, fonts, spacing, shadows
│   │   ├── [✓] globals.css                 # Reset, nav, footer, hero, auth pages, error page, toast, responsive
│   │   ├── [✓] components.css              # Buttons, forms, cards, badges, tables, modals, pagination, stars
│   │   ├── [✓] animations.css              # 8 keyframes + utility classes: pulse, scan, flicker, glitch
│   │   └── [ ] admin.css                   # Admin-specific layout: sidebar, dashboard grid, charts
│   │
│   ├── js/
│   │   ├── [✓] app.js                      # Toast mgmt, HTMX error handlers, keyboard shortcuts
│   │   └── [ ] charts.js                   # Revenue/profit chart rendering (analytics page)
│   │
│   ├── fonts/                               # Self-hosted fonts (fallback if Google Fonts CDN fails)
│   └── images/                              # Product images, logos, icons
│
│
│   ═══════════════════════════════════════
│   TESTS
│   ═══════════════════════════════════════
│
└── tests/
    ├── [ ] auth_tests.rs                    # Register, login, invalid creds, duplicate email
    ├── [ ] product_tests.rs                 # CRUD, search, sort, out-of-stock
    ├── [ ] category_tests.rs                # Add/remove categories
    ├── [ ] cart_tests.rs                    # Add/remove items, stock limit enforcement
    ├── [ ] order_tests.rs                   # Place, cancel, status transitions
    ├── [ ] review_tests.rs                  # Submit, approve, list approved only
    ├── [ ] wishlist_tests.rs                # Add/remove, discount notification trigger
    ├── [ ] payment_tests.rs                 # Mock bank flow, invoice generation
    ├── [ ] refund_tests.rs                  # 30-day window, discount-price preservation
    ├── [ ] delivery_tests.rs                # Create, mark complete, list
    ├── [ ] discount_tests.rs                # Apply, price recalculation, notification
    ├── [ ] analytics_tests.rs               # Revenue/profit calculation correctness
    └── [ ] stock_tests.rs                   # Concurrent decrement safety, restock on refund
```

## Summary

| Category    | Total files | Implemented | Remaining |
|-------------|-------------|-------------|-----------|
| Rust source | 63          | 14          | 49        |
| Templates   | 49          | 6           | 43        |
| Static      | 7           | 5           | 2         |
| Config/Docs | 8           | 6           | 2         |
| Tests       | 13          | 0           | 13        |
| **Total**   | **140**     | **31**      | **109**   |

## Requirement → File Mapping

| Req | Description                          | Key files                                                          |
|-----|--------------------------------------|--------------------------------------------------------------------|
| 1   | Products in categories, add to cart  | `products.rs`, `cart.rs`, `product_card.html`, `product_list.html` |
| 3   | Stock decrement, order status        | `stock_service.rs`, `order_service.rs`, `order_timeline.html`      |
| 4   | Login before order, invoice PDF+email| `auth.rs`, `auth_service.rs`, `pdf_service.rs`, `email_service.rs` |
| 5   | Ratings, comments, manager approval  | `reviews.rs`, `review_service.rs`, `comments.rs`, `star_rating.html` |
| 6   | Attractive GUI                       | `theme.css`, `globals.css`, `components.css`, `animations.css`     |
| 7   | Search, sort, out-of-stock handling  | `products.rs` (search partial), `search_results.html`, `stock_indicator.html` |
| 8   | Admin interface                      | `routes/admin/*`, `templates/admin/*`, `admin.css`                 |
| 9   | Product properties                   | `003_create_products.sql`, `models/product.rs`                     |
| 10  | Three roles                          | `user_role` enum, `role_guard.rs`, `RoleRoute`                     |
| 11  | Sales manager features               | `sales.rs`, `discounts.rs`, `invoices.rs`, `analytics.rs`, `notification_service.rs` |
| 12  | Product manager features             | `product_mgr.rs`, `deliveries.rs`, `categories.rs`, `stock.rs`, `comments.rs` |
| 13  | Customer features                    | `orders.rs`, `wishlist.rs`, `refunds.rs`, `reviews.rs`             |
| 14  | Credit card entry                    | `payments.rs`, `checkout.html`                                     |
| 15  | 30-day return + refund               | `refunds.rs`, `refund_service.rs`, `refund_mgr.rs`                 |
| 16  | Security & encryption                | `encryption_service.rs`, `auth.rs` (HttpOnly cookies), `role_guard.rs` |
| 17  | Concurrency                          | `pool.rs` (connection limits), `stock_service.rs` (row locking)    |

## Design Decisions

### Why HTMX over React/Vue?
- Zero npm dependencies. No node_modules, no build step, no framework churn.
- HTMX is 14KB gzipped. The entire JS payload is HTMX + 170 lines of app.js.
- Server-rendered HTML means the Rust backend owns all state and rendering logic. No API/frontend state sync bugs.
- Partials (`templates/partials/`) enable dynamic interactions (search, cart, modals) without full page reloads.

### Why Tera over Askama?
- Tera templates are loaded at runtime and can be hot-reloaded in development. Askama compiles templates into Rust code — faster in production but requires recompilation on every HTML change.
- Tera's Jinja2-like syntax is familiar to anyone who's used Django, Flask, or Hugo.

### Why argon2id over bcrypt?
- Argon2id is the current OWASP recommendation (2024). It's resistant to GPU attacks (memory-hard) and side-channel attacks. Bcrypt is still acceptable but has a 72-byte password limit.

### Why cookie-based JWT over session tokens?
- Cookies are sent automatically on every request, including HTMX partials. No JavaScript token management needed.
- `HttpOnly` flag prevents XSS from reading the token. `SameSite=Lax` prevents CSRF on POST requests.
- JWTs are stateless — no server-side session store needed. Trade-off: tokens can't be individually revoked without a blocklist.
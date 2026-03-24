#!/usr/bin/env bash
set -e

# Run from inside cs308_project/

# ============================================================
# TOP-LEVEL
# ============================================================
touch README.md
touch .gitignore
touch .env.example
touch docker-compose.yml

mkdir -p .github/workflows
touch .github/workflows/ci.yml

mkdir -p docs
touch docs/architecture.md
touch docs/api.md
touch docs/database_schema.md

# ============================================================
# BACKEND — Rust / Axum / Tera / HTMX
# ============================================================
touch Cargo.toml
touch Dockerfile
touch build.rs

# -- src root
mkdir -p src
touch src/main.rs
touch src/config.rs
touch src/errors.rs
touch src/lib.rs

# -- database
mkdir -p src/db
touch src/db/mod.rs
touch src/db/pool.rs

mkdir -p src/db/migrations
touch src/db/migrations/001_create_users.sql
touch src/db/migrations/002_create_categories.sql
touch src/db/migrations/003_create_products.sql
touch src/db/migrations/004_create_carts.sql
touch src/db/migrations/005_create_orders.sql
touch src/db/migrations/006_create_order_items.sql
touch src/db/migrations/007_create_reviews.sql
touch src/db/migrations/008_create_wishlists.sql
touch src/db/migrations/009_create_invoices.sql
touch src/db/migrations/010_create_deliveries.sql
touch src/db/migrations/011_create_refunds.sql
touch src/db/migrations/012_create_discounts.sql
touch src/db/migrations/013_create_notifications.sql

# -- models
mkdir -p src/models
touch src/models/mod.rs
touch src/models/user.rs
touch src/models/product.rs
touch src/models/category.rs
touch src/models/cart.rs
touch src/models/cart_item.rs
touch src/models/order.rs
touch src/models/order_item.rs
touch src/models/review.rs
touch src/models/wishlist.rs
touch src/models/invoice.rs
touch src/models/delivery.rs
touch src/models/refund.rs
touch src/models/discount.rs
touch src/models/notification.rs

# -- routes (serves both HTML pages and HTMX partials)
mkdir -p src/routes
touch src/routes/mod.rs
touch src/routes/auth.rs
touch src/routes/products.rs
touch src/routes/categories.rs
touch src/routes/cart.rs
touch src/routes/orders.rs
touch src/routes/reviews.rs
touch src/routes/wishlist.rs
touch src/routes/payments.rs
touch src/routes/refunds.rs
touch src/routes/pages.rs

mkdir -p src/routes/admin
touch src/routes/admin/mod.rs
touch src/routes/admin/sales.rs
touch src/routes/admin/product_mgr.rs
touch src/routes/admin/deliveries.rs
touch src/routes/admin/comments.rs
touch src/routes/admin/analytics.rs
touch src/routes/admin/invoices.rs
touch src/routes/admin/categories.rs
touch src/routes/admin/discounts.rs
touch src/routes/admin/stock.rs
touch src/routes/admin/refund_mgr.rs

# -- middleware
mkdir -p src/middleware
touch src/middleware/mod.rs
touch src/middleware/auth.rs
touch src/middleware/role_guard.rs
touch src/middleware/cors.rs

# -- services
mkdir -p src/services
touch src/services/mod.rs
touch src/services/auth_service.rs
touch src/services/product_service.rs
touch src/services/category_service.rs
touch src/services/cart_service.rs
touch src/services/order_service.rs
touch src/services/review_service.rs
touch src/services/wishlist_service.rs
touch src/services/payment_service.rs
touch src/services/refund_service.rs
touch src/services/pdf_service.rs
touch src/services/email_service.rs
touch src/services/notification_service.rs
touch src/services/delivery_service.rs
touch src/services/discount_service.rs
touch src/services/analytics_service.rs
touch src/services/encryption_service.rs
touch src/services/stock_service.rs

# -- tests
mkdir -p tests
touch tests/auth_tests.rs
touch tests/product_tests.rs
touch tests/category_tests.rs
touch tests/cart_tests.rs
touch tests/order_tests.rs
touch tests/review_tests.rs
touch tests/wishlist_tests.rs
touch tests/payment_tests.rs
touch tests/refund_tests.rs
touch tests/delivery_tests.rs
touch tests/discount_tests.rs
touch tests/analytics_tests.rs
touch tests/stock_tests.rs

# ============================================================
# TEMPLATES — Tera (.html)
# ============================================================

# -- base layouts
mkdir -p templates
touch templates/base.html
touch templates/base_admin.html

# -- reusable components (included via {% include %})
mkdir -p templates/components
touch templates/components/navbar.html
touch templates/components/footer.html
touch templates/components/sidebar.html
touch templates/components/product_card.html
touch templates/components/cart_item.html
touch templates/components/cart_drawer.html
touch templates/components/review_card.html
touch templates/components/review_form.html
touch templates/components/star_rating.html
touch templates/components/search_bar.html
touch templates/components/sort_dropdown.html
touch templates/components/pagination.html
touch templates/components/toast.html
touch templates/components/modal.html
touch templates/components/stock_indicator.html
touch templates/components/order_status_badge.html
touch templates/components/order_timeline.html
touch templates/components/invoice_view.html

# -- full pages (extend base.html)
mkdir -p templates/pages
touch templates/pages/home.html
touch templates/pages/login.html
touch templates/pages/register.html
touch templates/pages/product_list.html
touch templates/pages/product_detail.html
touch templates/pages/cart.html
touch templates/pages/checkout.html
touch templates/pages/payment_result.html
touch templates/pages/order_history.html
touch templates/pages/order_detail.html
touch templates/pages/wishlist.html
touch templates/pages/profile.html
touch templates/pages/refund_request.html
touch templates/pages/not_found.html

# -- admin pages (extend base_admin.html)
mkdir -p templates/admin
touch templates/admin/dashboard.html
touch templates/admin/manage_products.html
touch templates/admin/manage_categories.html
touch templates/admin/manage_orders.html
touch templates/admin/manage_deliveries.html
touch templates/admin/review_moderation.html
touch templates/admin/pricing_discounts.html
touch templates/admin/invoice_manager.html
touch templates/admin/revenue_analytics.html
touch templates/admin/refund_manager.html
touch templates/admin/stock_manager.html

# -- HTMX partials (fragments swapped via hx-get / hx-post)
mkdir -p templates/partials
touch templates/partials/product_grid.html
touch templates/partials/cart_summary.html
touch templates/partials/search_results.html
touch templates/partials/review_list.html
touch templates/partials/order_list.html
touch templates/partials/delivery_table.html
touch templates/partials/invoice_table.html
touch templates/partials/stock_table.html
touch templates/partials/comment_list.html
touch templates/partials/refund_list.html
touch templates/partials/discount_form.html
touch templates/partials/category_form.html
touch templates/partials/notification_list.html
touch templates/partials/revenue_chart.html
touch templates/partials/wishlist_items.html

# ============================================================
# STATIC ASSETS — served by Axum tower-http
# ============================================================
mkdir -p static/css
touch static/css/globals.css
touch static/css/theme.css
touch static/css/animations.css
touch static/css/components.css
touch static/css/admin.css

mkdir -p static/js
touch static/js/app.js
touch static/js/charts.js

mkdir -p static/fonts
mkdir -p static/images

# ============================================================
echo ""
echo "=== Done! ==="
TOTAL=$(find . -type f -not -path './.git/*' | wc -l)
DIRS=$(find . -type d -not -path './.git/*' | wc -l)
echo "Created $DIRS directories and $TOTAL files."
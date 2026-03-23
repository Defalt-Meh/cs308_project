#!/usr/bin/env bash
set -e

# ============================================================
# TOP-LEVEL
# ============================================================
touch README.md
touch .gitignore
touch .env.example
touch docker-compose.yml

# ============================================================
# CI / DOCS
# ============================================================
mkdir -p .github/workflows
touch .github/workflows/ci.yml

mkdir -p docs
touch docs/architecture.md
touch docs/api.md
touch docs/database_schema.md

# ============================================================
# BACKEND — Rust / Axum
# ============================================================
mkdir -p backend
touch backend/Cargo.toml
touch backend/Dockerfile
touch backend/.env.example

mkdir -p backend/src
touch backend/src/main.rs
touch backend/src/config.rs
touch backend/src/errors.rs
touch backend/src/lib.rs

mkdir -p backend/src/db
touch backend/src/db/mod.rs
touch backend/src/db/pool.rs

mkdir -p backend/src/db/migrations
touch backend/src/db/migrations/001_create_users.sql
touch backend/src/db/migrations/002_create_categories.sql
touch backend/src/db/migrations/003_create_products.sql
touch backend/src/db/migrations/004_create_carts.sql
touch backend/src/db/migrations/005_create_orders.sql
touch backend/src/db/migrations/006_create_order_items.sql
touch backend/src/db/migrations/007_create_reviews.sql
touch backend/src/db/migrations/008_create_wishlists.sql
touch backend/src/db/migrations/009_create_invoices.sql
touch backend/src/db/migrations/010_create_deliveries.sql
touch backend/src/db/migrations/011_create_refunds.sql
touch backend/src/db/migrations/012_create_discounts.sql
touch backend/src/db/migrations/013_create_notifications.sql

mkdir -p backend/src/models
touch backend/src/models/mod.rs
touch backend/src/models/user.rs
touch backend/src/models/product.rs
touch backend/src/models/category.rs
touch backend/src/models/cart.rs
touch backend/src/models/cart_item.rs
touch backend/src/models/order.rs
touch backend/src/models/order_item.rs
touch backend/src/models/review.rs
touch backend/src/models/wishlist.rs
touch backend/src/models/invoice.rs
touch backend/src/models/delivery.rs
touch backend/src/models/refund.rs
touch backend/src/models/discount.rs
touch backend/src/models/notification.rs

mkdir -p backend/src/routes
touch backend/src/routes/mod.rs
touch backend/src/routes/auth.rs
touch backend/src/routes/products.rs
touch backend/src/routes/categories.rs
touch backend/src/routes/cart.rs
touch backend/src/routes/orders.rs
touch backend/src/routes/reviews.rs
touch backend/src/routes/wishlist.rs
touch backend/src/routes/payments.rs
touch backend/src/routes/refunds.rs

mkdir -p backend/src/routes/admin
touch backend/src/routes/admin/mod.rs
touch backend/src/routes/admin/sales.rs
touch backend/src/routes/admin/product_mgr.rs
touch backend/src/routes/admin/deliveries.rs
touch backend/src/routes/admin/comments.rs
touch backend/src/routes/admin/analytics.rs
touch backend/src/routes/admin/invoices.rs
touch backend/src/routes/admin/categories.rs
touch backend/src/routes/admin/discounts.rs
touch backend/src/routes/admin/stock.rs
touch backend/src/routes/admin/refund_mgr.rs

mkdir -p backend/src/middleware
touch backend/src/middleware/mod.rs
touch backend/src/middleware/auth.rs
touch backend/src/middleware/role_guard.rs
touch backend/src/middleware/cors.rs

mkdir -p backend/src/services
touch backend/src/services/mod.rs
touch backend/src/services/auth_service.rs
touch backend/src/services/product_service.rs
touch backend/src/services/category_service.rs
touch backend/src/services/cart_service.rs
touch backend/src/services/order_service.rs
touch backend/src/services/review_service.rs
touch backend/src/services/wishlist_service.rs
touch backend/src/services/payment_service.rs
touch backend/src/services/refund_service.rs
touch backend/src/services/pdf_service.rs
touch backend/src/services/email_service.rs
touch backend/src/services/notification_service.rs
touch backend/src/services/delivery_service.rs
touch backend/src/services/discount_service.rs
touch backend/src/services/analytics_service.rs
touch backend/src/services/encryption_service.rs
touch backend/src/services/stock_service.rs

mkdir -p backend/tests
touch backend/tests/auth_tests.rs
touch backend/tests/product_tests.rs
touch backend/tests/category_tests.rs
touch backend/tests/cart_tests.rs
touch backend/tests/order_tests.rs
touch backend/tests/review_tests.rs
touch backend/tests/wishlist_tests.rs
touch backend/tests/payment_tests.rs
touch backend/tests/refund_tests.rs
touch backend/tests/delivery_tests.rs
touch backend/tests/discount_tests.rs
touch backend/tests/analytics_tests.rs
touch backend/tests/stock_tests.rs

# ============================================================
# FRONTEND — React / Vite / TypeScript
# ============================================================
mkdir -p frontend
touch frontend/package.json
touch frontend/tsconfig.json
touch frontend/tsconfig.node.json
touch frontend/vite.config.ts
touch frontend/index.html
touch frontend/Dockerfile
touch frontend/.eslintrc.cjs

mkdir -p frontend/public
touch frontend/public/favicon.svg

mkdir -p frontend/src
touch frontend/src/main.tsx
touch frontend/src/App.tsx
touch frontend/src/vite-env.d.ts

mkdir -p frontend/src/api
touch frontend/src/api/client.ts
touch frontend/src/api/auth.ts
touch frontend/src/api/products.ts
touch frontend/src/api/categories.ts
touch frontend/src/api/cart.ts
touch frontend/src/api/orders.ts
touch frontend/src/api/reviews.ts
touch frontend/src/api/wishlist.ts
touch frontend/src/api/payments.ts
touch frontend/src/api/refunds.ts
touch frontend/src/api/admin.ts
touch frontend/src/api/notifications.ts

mkdir -p frontend/src/types
touch frontend/src/types/index.ts
touch frontend/src/types/user.ts
touch frontend/src/types/product.ts
touch frontend/src/types/category.ts
touch frontend/src/types/cart.ts
touch frontend/src/types/order.ts
touch frontend/src/types/review.ts
touch frontend/src/types/invoice.ts
touch frontend/src/types/delivery.ts
touch frontend/src/types/refund.ts
touch frontend/src/types/discount.ts
touch frontend/src/types/notification.ts

mkdir -p frontend/src/store
touch frontend/src/store/authStore.ts
touch frontend/src/store/cartStore.ts
touch frontend/src/store/productStore.ts
touch frontend/src/store/notificationStore.ts

mkdir -p frontend/src/hooks
touch frontend/src/hooks/useAuth.ts
touch frontend/src/hooks/useCart.ts
touch frontend/src/hooks/useProducts.ts
touch frontend/src/hooks/useDebounce.ts
touch frontend/src/hooks/useClickOutside.ts

mkdir -p frontend/src/styles
touch frontend/src/styles/globals.css
touch frontend/src/styles/theme.ts
touch frontend/src/styles/fonts.css
touch frontend/src/styles/animations.css

mkdir -p frontend/src/components/ui
touch frontend/src/components/ui/Button.tsx
touch frontend/src/components/ui/Input.tsx
touch frontend/src/components/ui/Modal.tsx
touch frontend/src/components/ui/Card.tsx
touch frontend/src/components/ui/Badge.tsx
touch frontend/src/components/ui/Spinner.tsx
touch frontend/src/components/ui/StarRating.tsx
touch frontend/src/components/ui/SearchBar.tsx
touch frontend/src/components/ui/SortDropdown.tsx
touch frontend/src/components/ui/Pagination.tsx
touch frontend/src/components/ui/Toast.tsx
touch frontend/src/components/ui/ConfirmDialog.tsx
touch frontend/src/components/ui/DataTable.tsx

mkdir -p frontend/src/components/layout
touch frontend/src/components/layout/Navbar.tsx
touch frontend/src/components/layout/Sidebar.tsx
touch frontend/src/components/layout/Footer.tsx
touch frontend/src/components/layout/MainLayout.tsx
touch frontend/src/components/layout/AdminLayout.tsx
touch frontend/src/components/layout/ProtectedRoute.tsx
touch frontend/src/components/layout/RoleRoute.tsx

mkdir -p frontend/src/components/product
touch frontend/src/components/product/ProductCard.tsx
touch frontend/src/components/product/ProductGrid.tsx
touch frontend/src/components/product/ProductDetail.tsx
touch frontend/src/components/product/StockIndicator.tsx

mkdir -p frontend/src/components/cart
touch frontend/src/components/cart/CartItem.tsx
touch frontend/src/components/cart/CartSummary.tsx
touch frontend/src/components/cart/CartDrawer.tsx

mkdir -p frontend/src/components/order
touch frontend/src/components/order/OrderCard.tsx
touch frontend/src/components/order/OrderStatusBadge.tsx
touch frontend/src/components/order/OrderTimeline.tsx
touch frontend/src/components/order/InvoiceView.tsx

mkdir -p frontend/src/components/review
touch frontend/src/components/review/ReviewForm.tsx
touch frontend/src/components/review/ReviewList.tsx
touch frontend/src/components/review/ReviewCard.tsx

mkdir -p frontend/src/components/admin
touch frontend/src/components/admin/StatsCard.tsx
touch frontend/src/components/admin/RevenueChart.tsx
touch frontend/src/components/admin/DeliveryTable.tsx
touch frontend/src/components/admin/CommentModerationCard.tsx
touch frontend/src/components/admin/DiscountForm.tsx
touch frontend/src/components/admin/InvoiceTable.tsx
touch frontend/src/components/admin/RefundRequestCard.tsx
touch frontend/src/components/admin/StockTable.tsx
touch frontend/src/components/admin/CategoryForm.tsx

mkdir -p frontend/src/pages
touch frontend/src/pages/Home.tsx
touch frontend/src/pages/Login.tsx
touch frontend/src/pages/Register.tsx
touch frontend/src/pages/ProductList.tsx
touch frontend/src/pages/ProductDetail.tsx
touch frontend/src/pages/Cart.tsx
touch frontend/src/pages/Checkout.tsx
touch frontend/src/pages/PaymentResult.tsx
touch frontend/src/pages/OrderHistory.tsx
touch frontend/src/pages/OrderDetail.tsx
touch frontend/src/pages/Wishlist.tsx
touch frontend/src/pages/Profile.tsx
touch frontend/src/pages/RefundRequest.tsx
touch frontend/src/pages/NotFound.tsx

mkdir -p frontend/src/pages/admin
touch frontend/src/pages/admin/Dashboard.tsx
touch frontend/src/pages/admin/ManageProducts.tsx
touch frontend/src/pages/admin/ManageCategories.tsx
touch frontend/src/pages/admin/ManageOrders.tsx
touch frontend/src/pages/admin/ManageDeliveries.tsx
touch frontend/src/pages/admin/ReviewModeration.tsx
touch frontend/src/pages/admin/PricingDiscounts.tsx
touch frontend/src/pages/admin/InvoiceManager.tsx
touch frontend/src/pages/admin/RevenueAnalytics.tsx
touch frontend/src/pages/admin/RefundManager.tsx
touch frontend/src/pages/admin/StockManager.tsx

mkdir -p frontend/src/utils
touch frontend/src/utils/formatCurrency.ts
touch frontend/src/utils/formatDate.ts
touch frontend/src/utils/validators.ts
touch frontend/src/utils/constants.ts
touch frontend/src/utils/cn.ts

mkdir -p frontend/src/__tests__
touch frontend/src/__tests__/Login.test.tsx
touch frontend/src/__tests__/ProductCard.test.tsx
touch frontend/src/__tests__/Cart.test.tsx
touch frontend/src/__tests__/SearchBar.test.tsx

# ============================================================
echo ""
echo "=== Done! ==="
TOTAL=$(find . -type f -not -path './.git/*' | wc -l)
DIRS=$(find . -type d -not -path './.git/*' | wc -l)
echo "Created $DIRS directories and $TOTAL files."

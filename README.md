# CS 308 Online Store

> Sabancı University — CS 308 Software Engineering Course Project

A full-stack e-commerce platform built as a single Rust binary. The server
renders HTML with **Tera** templates, uses **HTMX** for dynamic interactions,
and stores data in **PostgreSQL**. No JavaScript framework — just one
`<script>` tag and 170 lines of vanilla JS.

## Tech Stack

| Layer       | Technology                              |
|-------------|-----------------------------------------|
| Language    | Rust 1.78+                              |
| Framework   | Axum 0.7                                |
| Templates   | Tera (Jinja2-style)                     |
| Interactivity | HTMX 2.0                              |
| Database    | PostgreSQL 16 via sqlx                  |
| Auth        | JWT (HttpOnly cookies) + argon2id       |
| PDF         | genpdf                                  |
| Email       | lettre                                  |
| Styling     | Custom CSS (Samaritan theme)            |
| Deployment  | Docker + docker-compose                 |

## Quick Start

### Prerequisites
- Rust toolchain (`rustup`)
- PostgreSQL 16 (or use Docker)
- `sqlx-cli` (optional, for manual migrations)

### With Docker (recommended)
```bash
docker-compose up -d        # Starts Postgres + backend
open http://localhost:8080   # Browse the store
```

### Without Docker
```bash
# 1. Start Postgres and create the database
createdb store_db

# 2. Configure environment
cp .env.example .env
# Edit .env with your DATABASE_URL and JWT_SECRET

# 3. Run
cargo run
```

The server starts at `http://localhost:8080`. Migrations run
automatically on startup.

## Project Structure

See [docs/architecture.md](docs/architecture.md) for the full
annotated file tree and design decisions.

```
cs308_project/
├── src/            ← Rust: routes, services, models, middleware
├── templates/      ← Tera HTML: pages, components, HTMX partials
├── static/         ← CSS, JS, images (served by tower-http)
├── tests/          ← Integration tests
├── docs/           ← Architecture docs, API reference
└── Cargo.toml      ← One `cargo run` and you're live
```

## Roles

| Role              | Capabilities                                           |
|-------------------|--------------------------------------------------------|
| Customer          | Browse, search, cart, order, review, wishlist, returns  |
| Sales Manager     | Pricing, discounts, invoices, revenue analytics         |
| Product Manager   | Stock, categories, deliveries, comment approval         |

## Sprint Schedule

| Date       | Event                                      |
|------------|--------------------------------------------|
| 13.03.2026 | Sprint 1 Planning + Start                  |
| 27.03.2026 | Sprint 1 Review & Sprint 2 Start           |
| 10.04.2026 | Sprint 2 Review & Sprint 3 Start           |
| 24.04.2026 | Sprint 3 Review & Sprint 4 Start           |
| 01.05.2026 | Progress Demo (Req 1,3,4,5,7,9)            |
| 15.05.2026 | Sprint 4 Review & Sprint 5 Start           |
| 22.05.2026 | Sprint 5 Review                             |
| TBA        | Final Demo (all requirements)               |

## License

Course project — not for redistribution.
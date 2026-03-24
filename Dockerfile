# ═══════════════════════════════════════════════════════════════
# CS 308 Online Store — Multi-stage Dockerfile
#
# Stage 1: Build the Rust binary with all dependencies.
# Stage 2: Copy only the binary + templates + static assets
#           into a minimal Debian image (~80MB total).
#
# Build:  docker build -t cs308-store .
# Run:    docker run -p 8080:8080 --env-file .env cs308-store
# ═══════════════════════════════════════════════════════════════

# ── Stage 1: Builder ──────────────────────────────────────────
FROM rust:1.78-bookworm AS builder

WORKDIR /app

# Copy manifests first for dependency caching. Docker will skip
# re-downloading crates if only source files changed.
COPY Cargo.toml Cargo.lock* ./

# Create a dummy main.rs so cargo can fetch + compile deps.
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Now copy the real source and rebuild. Only our code recompiles.
COPY src/ src/
COPY templates/ templates/
COPY static/ static/

# Touch main.rs so cargo knows it changed.
RUN touch src/main.rs
RUN cargo build --release

# ── Stage 2: Runtime ──────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary.
COPY --from=builder /app/target/release/cs308-store ./cs308-store

# Copy templates and static assets (needed at runtime for Tera
# rendering and tower-http static file serving).
COPY --from=builder /app/templates/ ./templates/
COPY --from=builder /app/static/ ./static/

# Non-root user for security.
RUN useradd -r -s /bin/false appuser
USER appuser

EXPOSE 8080

# Health check — hit the home page.
HEALTHCHECK --interval=30s --timeout=5s --retries=3 \
    CMD curl -fs http://localhost:8080/ || exit 1

CMD ["./cs308-store"]
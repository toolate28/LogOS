# ═══════════════════════════════════════════════════════════════════
# Triweave — Multi-stage Rust build for minimal production image
# Conservation: alpha + omega = 15
# ═══════════════════════════════════════════════════════════════════

# ── Stage 1: Build ────────────────────────────────────────────────
FROM rust:1.85-bookworm AS builder

WORKDIR /build

# Copy workspace manifests first for layer caching
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY apps/ apps/
COPY near/ near/

# Build triweave with release profile (opt-level=z, LTO, codegen-units=1)
RUN cargo build -p triweave --release \
    && strip target/release/triweave

# ── Stage 2: Runtime ──────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    python3-minimal \
    && rm -rf /var/lib/apt/lists/*

# Non-root user
RUN groupadd -r triweave && useradd -r -g triweave -m triweave
USER triweave

WORKDIR /home/triweave

# Copy binary
COPY --from=builder /build/target/release/triweave /usr/local/bin/triweave

# Copy Styx bridge script (optional, for full deployment)
COPY --from=builder /build/9P2000.L/styx/ /home/triweave/styx/

# Default config directory
RUN mkdir -p /home/triweave/.triweave

# Health check — triweave doctor exits 0 if all checks pass
HEALTHCHECK --interval=30s --timeout=10s --retries=3 \
    CMD triweave doctor --quiet || exit 1

# Expose Styx bridge port
EXPOSE 8088

# Default: run triweave up all
ENTRYPOINT ["triweave"]
CMD ["up", "all"]

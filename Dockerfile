# Builder stage for the webui
FROM node:22-slim AS webui-builder
WORKDIR /usr/src/webui
RUN npm install -g pnpm
COPY webui/package.json webui/pnpm-lock.yaml* ./
RUN pnpm install
COPY webui/ ./
RUN pnpm build

# Builder stage for Rust
FROM rust:1.85-slim AS builder
WORKDIR /usr/src/app
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock* ./
COPY src/ ./src/
COPY --from=webui-builder /usr/src/webui/dist ./webui/dist
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app
# RUN apt-get update && \
#     apt-get install -y ca-certificates && \
#     rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/colink /app/colink
EXPOSE 3000
CMD ["./colink", "serve"]

FROM rust:1.76.0 AS builder

WORKDIR /app

RUN apt update && apt install lld clang -y
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/circuit_chronicle /app
COPY ./configuration /app/configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./circuit_chronicle"]

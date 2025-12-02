FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
COPY templates ./templates
RUN cargo build --release

FROM scratch
COPY --from=builder /app/target/release/placeholder-web /placeholder-web
COPY ./assets /assets
COPY ./templates /templates

EXPOSE 8080
ENTRYPOINT ["/placeholder-web"]

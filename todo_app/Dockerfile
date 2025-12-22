FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev

WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine
WORKDIR /app
COPY --from=builder /app/target/release/todo_app /app/todo_app
ENTRYPOINT ["/app/todo_app"]

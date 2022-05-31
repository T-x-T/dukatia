FROM rustlang/rust:nightly AS builder

WORKDIR /app

ADD . ./
RUN cargo build --release

FROM debian:bullseye-slim
EXPOSE 4000

COPY --from=builder /app/target/release/backend /app/backend
WORKDIR /app
CMD ["/app/backend"]
FROM rustlang/rust:nightly AS builder

WORKDIR /app

ADD . ./
RUN rustup default nightly-2023-06-09 && cargo build --release

FROM debian:bullseye-slim
EXPOSE 4000

COPY --from=builder /app/target/release/backend /app/backend
ADD sql /app/sql

CMD ["/app/backend"]
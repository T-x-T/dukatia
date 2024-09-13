FROM alpine AS builder

WORKDIR /app

ADD . ./
RUN apk add --no-cache rustup build-base && /usr/bin/rustup-init -y --default-toolchain nightly-unknown-linux-musl --profile minimal && ~/.cargo/bin/cargo build --release

FROM alpine
EXPOSE 4001

COPY --from=builder /app/target/release/demo_account_creator /app/demo_account_creator

CMD ["/app/demo_account_creator"]
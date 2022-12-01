FROM --platform=$BUILDPLATFORM rust:1.65 AS rust
ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
  "linux/arm/v7") echo armv7-unknown-linux-gnueabihf > /rust_target.txt ;; \
  "linux/arm/v6") echo armv-unknown-linux-gnueabihf > /rust_target.txt ;; \
  *) exit 1 ;; \
esac
RUN rustup target add $(cat /rust_target.txt)
RUN apt-get update && apt-get -y install binutils-arm-linux-gnueabihf librust-openssl-sys-dev libssl-dev gcc-arm-linux-gnueabihf pkg-config
WORKDIR /app
COPY .cargo ./.cargo
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --target $(cat /rust_target.txt)
RUN cp target/$(cat /rust_target.txt)/release/porkbun-ddns .

FROM alpine:3.12
WORKDIR /app
COPY --from=rust /app/porkbun-ddns ./
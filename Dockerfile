# Get started with a build env with Rust nightly
FROM docker.io/rustlang/rust:nightly-alpine AS builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen protoc openssl-dev openssl-libs-static musl-dev

RUN npm install -g sass

#RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh
RUN curl --proto '=https' --tlsv1.2 -LsSf https://leptos-rs.artifacts.axodotdev.host/cargo-leptos/v0.2.26/cargo-leptos-installer.sh | sh
# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

RUN cargo leptos build --release -vv

#FROM docker.io/rustlang/rust:nightly-alpine as runner
FROM gcr.io/distroless/cc AS runtime

WORKDIR /grpc_front

COPY --from=builder /work/target/release/grpc_front /grpc_front/
COPY --from=builder /work/target/site /grpc_front/site
COPY --from=builder /work/Cargo.toml /grpc_front/
COPY --from=builder /work/config/* /grpc_front/config/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site
ENV CONF_DIR=/grpc_front/config
EXPOSE 8080

CMD ["/grpc_front/grpc_front"]

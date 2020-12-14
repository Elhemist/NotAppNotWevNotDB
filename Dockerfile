FROM rustlang/rust:nightly-buster as builder

WORKDIR /product-delivery
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

# RUN apk add -U \
#     bash \
#     build-base \
#     coreutils \
#     curl \
#     cyrus-sasl-dev \
#     git \
#     libevent \
#     # libressl2.6-libcrypto \
#     # libressl2.6-libssl \
#     libsasl \
#     lz4-dev \
#     openssh \
#     openssl \
#     openssl-dev \
#     # python \
#     yajl-dev \
#     zlib-dev

RUN cargo build --release

FROM debian:buster-slim

EXPOSE 8000

ENV TZ=Etc/UTC 

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata libpq-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /product-delivery/target/release/product-delivery .

ENTRYPOINT ["./product-delivery"]
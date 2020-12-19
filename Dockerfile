FROM rustlang/rust:nightly-buster as builder

WORKDIR /product_delivery
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./migrations ./migrations

RUN cargo build --release

FROM debian:buster-slim

EXPOSE 3000

ENV TZ=Etc/UTC 

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata libpq-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /product_delivery/target/release/product_delivery .

ENTRYPOINT ["./product_delivery"]
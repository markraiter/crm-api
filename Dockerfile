FROM rust:1.53 as builder
WORKDIR /usr/src/crm_api
COPY . .
RUN apt-get update && apt-get install -y libssl-dev pkg-config libclang-dev clang
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/crm_api /usr/local/bin/crm_api
CMD ["crm_api"]

ARG RUST_VERSION=1.57
ARG ALPINE_VERSION=3.14

FROM rust:${RUST_VERSION}-alpine${ALPINE_VERSION} AS builder
RUN apk --no-cache add ca-certificates musl-dev && \
    update-ca-certificates

WORKDIR /server
COPY ./server /

RUN cargo build --release && \
    strip --strip-all -o /target/release/server-stripped /target/release/server

FROM scratch
COPY --from=builder /target/release/server-stripped /server

ENTRYPOINT ["/server"]
EXPOSE 8888


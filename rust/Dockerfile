ARG RUST_VERSION
ARG ALPINE_VERSION

FROM rust:${RUST_VERSION}-alpine${ALPINE_VERSION} AS builder
RUN apk update \
    && apk upgrade \
    && apk add --no-cache \
        ca-certificates \
        musl-dev \
    && update-ca-certificates 2>/dev/null || true

WORKDIR /server
COPY ./server /

RUN cargo build --release && \
    ls -l /target/release/ && \
    strip --strip-all -o /target/release/server-stripped /target/release/server

FROM scratch
COPY --from=builder /target/release/server-stripped /server

ENTRYPOINT ["/server"]
EXPOSE 8888


ARG BUILDER_IMAGE=fission/builder
ARG RUST_VERSION
ARG ALPINE_VERSION

FROM ${BUILDER_IMAGE}

FROM rust:${RUST_VERSION}-alpine${ALPINE_VERSION}

RUN apk update && \
    apk add jq musl-dev

COPY --from=0 /builder /builder
ADD build.sh /usr/local/bin/build

EXPOSE 8001
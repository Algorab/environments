-include ../rules.mk

PLATFORMS = linux/amd64,linux/arm64/v8

REPO = glutamat

.PHONY: all
all: rust-env-img rust-env-1.57-img

rust-env-img-buildargs := --build-arg RUST_VERSION=1.57 --build-arg ALPINE_VERSION=3.14

rust-env-1.57-img-buildargs := --build-arg RUST_VERSION=1.57 --build-arg ALPINE_VERSION=3.14

rust-env-img: Dockerfile

rust-env-1.57-img: Dockerfile
-include ../../rules.mk

PLATFORMS = linux/amd64#,linux/arm64/v8
REPO = glutamat

.PHONY: all
all: rust-builder-img rust-builder-1.57-img

rust-builder-img-buildargs := --build-arg RUST_VERSION=1.57 --build-arg ALPINE_VERSION=3.14

rust-builder-1.57-img-buildargs := --build-arg RUST_VERSION=1.57

rust-builder-img: Dockerfile

rust-builder-1.57-img: Dockerfile
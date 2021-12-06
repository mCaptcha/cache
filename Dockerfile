ARG REDIS_VER=6.2.6

# stretch|bionic|buster
ARG OSNICK=bullseye

# ARCH=x64|arm64v8|arm32v7
ARG ARCH=x64

FROM rust:1.57-bullseye as builder
WORKDIR /src
RUN set -ex; \
    apt-get update; \
    DEBIAN_FRONTEND=noninteractive \
    apt-get install -y --no-install-recommends redis clang gcc
COPY Cargo.toml Cargo.lock /src/
RUN mkdir -p /src/src && touch /src/src/lib.rs
RUN cargo build --release 
COPY . .
RUN cargo build --release 


#FROM redisfab/redis:${REDIS_VER}-${ARCH}-${OSNICK}
FROM redis:${REDIS_VER}-${OSNICK}

ARG REDIS_VER

ENV LIBDIR /usr/lib/redis/modules
WORKDIR /data
RUN mkdir -p "$LIBDIR"

COPY --from=builder /src/target/release/libcache.so "$LIBDIR"

EXPOSE 6379
CMD ["redis-server", "--loadmodule", "/usr/lib/redis/modules/libcache.so"]

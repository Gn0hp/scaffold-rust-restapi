FROM rustlang/rust:nightly-slim as build

WORKDIR /app

# install libpq and create new empty binary project
RUN apt-get update; \
    apt-get install -y --no-install-recommends libpq-dev; \
    rm -rf /var/lib/apt/lists/*; \
    USER=root cargo new --bin app

# copy manifests
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml .

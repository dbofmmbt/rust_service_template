# MUST be the same as the crate's name
ARG APP_NAME={{crate_name}}

ARG FOLDER=/usr/src/${APP_NAME}

FROM rust:1.63 as base
RUN rustup component add rustfmt clippy

# Define base folder
ARG FOLDER
WORKDIR ${FOLDER}

# System build-dependencies
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

FROM base as deps_builder
ARG APP_NAME

# Copy dependencies
RUN cargo init
COPY Cargo.toml Cargo.lock ./

# Building only dependencies
RUN cargo build --release --bin ${APP_NAME} --tests \
    && rm src/*.rs target/release/deps/${APP_NAME}*

FROM deps_builder as inspections

# Copying folders needed for CI
COPY src/ ./src
COPY tests/ ./tests
COPY settings/ ./settings

RUN cargo fmt --check
RUN cargo clippy --release -- -Dwarnings
RUN cargo test --release

FROM inspections as builder
ARG APP_NAME
# Building whole application
RUN cargo build --release --bin ${APP_NAME}

FROM debian:buster-slim as production 

ARG FOLDER
ARG APP_NAME

RUN useradd app
USER app

# Get binary from builder
COPY --from=builder --chown=app  ${FOLDER}/target/release/${APP_NAME} ./app

CMD ./app
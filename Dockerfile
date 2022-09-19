# MUST be the same as the crate's name
ARG APP_NAME=service_template  # TODO update app name

ARG FOLDER=/usr/src/${APP_NAME}

FROM rust:1.63 as base

# Define base folder
ARG FOLDER
WORKDIR ${FOLDER}

# System build dependencies
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Copy dependencies
RUN cargo init
COPY Cargo.toml Cargo.lock ./

FROM base as builder

ARG APP_NAME

# Building only dependencies
RUN cargo build --release \
    && rm src/*.rs target/release/deps/${APP_NAME}*

COPY . .

# Building whole application
RUN cargo build --release

FROM debian:buster-slim as production 

ARG FOLDER
ARG APP_NAME

RUN useradd app
USER app

# Get binary from builder
COPY --from=builder --chown=app  ${FOLDER}/target/release/${APP_NAME} ./app

CMD ./app
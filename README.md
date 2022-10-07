# Rust Service Template

This is the Rust Service Template. It is a project template for microservices.

## How to run

- Install rust on <http://rustup.rs>
- Run `cargo test` to build and run tests.
- Run `cargo run` to start the application.

## Features checklist

### Ready

- logs
  - tracing + opentelemetry
- Some way to integrate with new relic
  - new relic supports collecting OTLP data directly.
- health check
  - `/health`
- Trace Context propagation
  - `axum_tracing_opentelemetry` and `opentelemetry` propagators did it.
- Ready to use AWS (`aws_config`)
  - can we let more stuff done?
- Config through e.g. yaml or TOML and env vars override
  - we'll probably use `config`
- basic setup for service tests
  - using `wiremock`

### To-do

- Deploy files
  - Dockerfile (done)
  - taskdef
  - appspec
  - buildspec

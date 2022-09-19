# Rust Service Template

This is the Rust Service Template. It is a project template for microservices.

## How to run

- Install rust on <http://rustup.rs>
- Run `cargo test` to build and run tests.
- Run `cargo run` to start the application.

## Features checklist

- [x] logs
  - tracing + opentelemetry
- [x] Some way to integrate wit new relic
  - new relic supports collecting OTLP data directly.
- [x] health check
  - `/health`
- [ ] basic setup for service tests
  - evaluating `wiremock`
- [ ] config through e.g. yaml or TOML and env vars override
  - we'll probably use `config`
- [x] Trace Context propagation
  - `axum_tracing_opentelemetry` and `opentelemetry` propagators did it.
- [x] Ready to use AWS (`aws_config`)
  - can we let more stuff done?
- [ ] Deploy files
  - [x] Dockerfile
  - [ ] taskdef
  - [ ] appspec
  - [ ] buildspec

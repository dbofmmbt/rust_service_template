# Rust Service Template

This is the Rust Service Template. It is a project template for microservices.

Initially, this template is focused on HTTP services.

## How to run

- Install rust on <http://rustup.rs>
- Run `cargo test` to build and run tests.
- Run `cargo run` to start the application.

## Features checklist

### Ready

- logs
  - tracing + opentelemetry
- health check
  - `/health`
- Trace Context propagation
  - `axum_tracing_opentelemetry` and `opentelemetry` propagators did it.
- Config through e.g. yaml or TOML and env vars override
  - Using `config` for that
- basic setup for service tests
  - using `wiremock`
- Dockerfile
- Configured HTTP client
  - request and response tracing
  - trace context propagation

### TODO

- Use Docker's buildkit to cache dependencies
- evaluate addition of the `http_problem` crate
- Add endpoint to print information similar to [actuator info](https://docs.spring.io/spring-boot/docs/current/actuator-api/htmlsingle/#info)
- evaluate crates that generate OpenAPI schemas

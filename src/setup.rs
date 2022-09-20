use config::Config;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use tracing::metadata::LevelFilter;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, Layer, Registry};

pub(crate) fn telemetry() {
    // pipeline to process tracing events
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_batch(opentelemetry::runtime::Tokio)
        .unwrap();

    // Create a `tracing` layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // Using the `Registry` to compose multiple layers interested on `tracing` spans and events.
    let subscriber = Registry::default()
        .with(tracing_subscriber::fmt::layer().with_filter(LevelFilter::INFO)) // logs to stdout
        .with(telemetry); // exports telemetry data

    // Defines a global provider for trace context injector/extractor
    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to set the tracing subscriber");
}

pub fn config_setup() -> AppConfig {
    Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("Config"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap()
        .try_deserialize::<AppConfig>()
        .unwrap()
}

#[derive(serde::Deserialize, Debug)]
pub struct AppConfig {
    pub debug:bool,
    pub text:String
}

mod telemetry;
use config::Config;
use eyre::Context;
use service_template::Settings;
pub use telemetry::telemetry;
use tracing::info;

pub fn config_setup() -> eyre::Result<Settings> {
    let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "local".to_string());

    info!("Running with mode: {run_mode}");

    Config::builder()
        // Add in `./default.toml`
        .add_source(config::File::with_name("settings/default"))
        .add_source(config::File::with_name(&format!("settings/{run_mode}")).required(false))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .wrap_err("Couldn't build config from the sources")?
        .try_deserialize::<Settings>()
        .wrap_err("Couldn't load config into the `Settings` struct")
}

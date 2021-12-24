mod protocol;
mod systems;

pub use crate::protocol::temperature::temperature_service_client;
use crate::protocol::temperature::temperature_service_server::TemperatureServiceServer;
use crate::systems::temperature::TemperatureSensor;

use tonic::transport::Server;

use color_eyre::Result;
use tracing::{debug, info, instrument};
use tracing_subscriber::prelude::*;

/// Initialize error reporter and tracing via opentelemetry.
#[instrument]
fn setup() -> Result<()> {
    color_eyre::install()?;

    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("suitware-server")
        .install_simple()?;
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    tracing_subscriber::registry()
        .with(opentelemetry)
        .try_init()?;

    Ok(())
}

/// Print debug information before the application starts.
#[instrument]
fn debug_info() -> Result<()> {
    let date = env!("VERGEN_BUILD_TIMESTAMP");
    let branch = env!("VERGEN_GIT_BRANCH");
    let commit = env!("VERGEN_GIT_SHA");
    let version = env!("VERGEN_GIT_SEMVER");

    debug!(
        "Running suitware at build {}, version {}, branch {}, hash {}.",
        date, version, branch, commit
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    setup()?;

    let addr = "[::1]:50051".parse()?;
    info!("Starting suitware server on {}", addr);

    let temperature = TemperatureSensor::default();

    Server::builder()
        .add_service(TemperatureServiceServer::new(temperature))
        .serve(addr)
        .await?;

    Ok(())
}

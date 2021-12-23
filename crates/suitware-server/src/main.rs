mod protocol;
mod systems;

use crate::protocol::temperature::temperature_service_server::TemperatureServiceServer;
use crate::systems::temperature::TemperatureSensor;
pub use crate::protocol::temperature::temperature_service_client;

use tonic::transport::Server;

use color_eyre::Result;
use tracing::info;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize error reporter and tracing
    color_eyre::install()?;
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("report_example")
        .install_simple()?;
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    tracing_subscriber::registry()
        .with(opentelemetry)
        .try_init()?;

    // Some debug information
    let date = env!("VERGEN_BUILD_TIMESTAMP");
    let branch = env!("VERGEN_GIT_BRANCH");
    let commit = env!("VERGEN_GIT_SHA");
    let version = env!("VERGEN_GIT_SEMVER");
    info!(
        "Running suitware at build {}, version {}, branch {}, hash {}.",
        date, version, branch, commit
    );

    // Set the initial address we want to serve on
    let addr = "[::1]:50051".parse()?;

    info!("Starting suitware server on {}", addr);

    let temperature = TemperatureSensor::default();

    // Build and start the server
    Server::builder().add_service(TemperatureServiceServer::new(temperature))
        .serve(addr)
        .await?;

    Ok(())
}

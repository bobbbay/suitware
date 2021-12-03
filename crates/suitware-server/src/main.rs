mod protocol;
mod systems;

use crate::protocol::temperature::temperature_service_server::TemperatureServiceServer;
use crate::systems::temperature::TemperatureSensor;

use tonic::transport::Server;

use color_eyre::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize error reporter and tracing
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

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
    let addr = "[::1]:50051".parse().unwrap();

    info!("Starting suitware server on {}", addr);

    let temperature = TemperatureSensor::default();

    // Build and start the server
    Server::builder()
        .add_service(TemperatureServiceServer::new(temperature))
        .serve(addr)
        .await?;

    Ok(())
}

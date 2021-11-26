mod protocol;
mod systems;

use crate::systems::temperature::TemperatureSensor;
use crate::protocol::temperature::temperature_service_server::TemperatureServiceServer;

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
    info!("Running suitware at build {}, branch {}, hash {}", date, branch, commit);

    // Set the initial address we want to serve on
    let addr = "[::1]:50051".parse().unwrap();

    info!("Starting suitware server on {}", addr);

    // Build and start the server
    Server::builder()
        .add_service(TemperatureServiceServer::new(TemperatureSensor::default()))
        .serve(addr)
        .await?;

    Ok(())
}

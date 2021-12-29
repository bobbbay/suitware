use color_eyre::Result;
use tonic::transport::Server;
use tracing::{info, instrument};
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;

pub mod protocol;
pub mod systems;

use protocol::temperature::temperature_service_server::TemperatureServiceServer;
use systems::temperature::TemperatureSensor;

/// Initialize error reporter and tracing via Opentelemetry.
#[instrument]
fn install_tracing() -> Result<()> {
    color_eyre::install()?;

    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("suitware-server")
        .install_simple()?;
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(opentelemetry)
        .with(ErrorLayer::default())
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

    info!(
        "Running suitware at build {}, version {}, branch {}, hash {}.",
        date, version, branch, commit
    );

    Ok(())
}

struct Suitware {
    systems: [&dyn System],
    tasks: Option<TaskPool>,
}

trait System {}

#[instrument]
#[tokio::main]
async fn main() -> Result<()> {
    install_tracing()?;
    debug_info()?;

    let addr = "[::1]:50051".parse()?;
    info!("Starting suitware server on {}", addr);

    let temperature_sensor = TemperatureSensor::default();

    Server::builder()
        .add_service(TemperatureServiceServer::new(temperature_sensor))
        .serve(addr)
        .await?;

    Ok(())
}

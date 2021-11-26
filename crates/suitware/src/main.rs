mod protocol;
mod systems;

use crate::systems::temperature::{TemperatureSensor};
use tonic::transport::Server;
use protocol::temperature::temperature_service_server::TemperatureServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    println!("TemperatureSensor server listening on {}", addr);

    Server::builder()
        .add_service(TemperatureServiceServer::new(TemperatureSensor::default()))
        .serve(addr)
        .await?;

    Ok(())
}

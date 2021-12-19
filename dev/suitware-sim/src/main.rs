pub mod protocol;

use tonic::{Response, transport::Server};

use crate::protocol::temperature::temperature_service_server::{
    TemperatureService, TemperatureServiceServer,
};
use crate::protocol::temperature::Temperature;

#[derive(Default)]
pub struct TemperatureSensor {}

// TODO: We can connect this to an actual simulation model to run in reality.

#[tonic::async_trait]
impl TemperatureService for TemperatureSensor {
    async fn get_temperature(
        &self,
        _request: tonic::Request<protocol::temperature::TemperatureRequest>,
    ) -> Result<tonic::Response<protocol::temperature::Temperature>, tonic::Status> {
	let reply = Temperature { temperature: 20, target_temperature: 22 };
	
	Ok(Response::new(reply))
    }

    async fn set_temperature(
        &self,
        _request: tonic::Request<protocol::temperature::TargetTemperatureRequest>,
    ) -> Result<tonic::Response<protocol::temperature::Temperature>, tonic::Status> {
	let reply = Temperature { temperature: 20, target_temperature: 22 };
	
	Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50061".parse().unwrap();
    let greeter = TemperatureSensor::default();

    println!("Simulation server listening on {}", addr);

    Server::builder()
        .add_service(TemperatureServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}

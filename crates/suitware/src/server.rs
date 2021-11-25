use tonic::{transport::Server, Request, Response, Status};

use temperature::temperature_service_server::{TemperatureService, TemperatureServiceServer};
use temperature::{Temperature, TemperatureRequest, TargetTemperatureRequest};

pub mod temperature {
    tonic::include_proto!("temperature");
}

#[derive(Default)]
pub struct TemperatureSensor {}

#[tonic::async_trait]
impl TemperatureService for TemperatureSensor {
    async fn get_temperature(
        &self,
        request: Request<TemperatureRequest>,
    ) -> Result<Response<Temperature>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = Temperature {
            temperature: 1,
            target_temperature: 5,
        };
        Ok(Response::new(reply))
    }

    async fn set_temperature(
        &self,
        request: Request<TargetTemperatureRequest>,
    ) -> Result<Response<Temperature>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = Temperature {
            temperature: 1,
            target_temperature: 5,
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = TemperatureSensor::default();

    println!("SemperatureSensor server listening on {}", addr);

    Server::builder()
        .add_service(TemperatureServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}

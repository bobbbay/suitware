use tonic::{Request, Response, Status};

use crate::protocol::temperature::temperature_service_server::TemperatureService;
use crate::protocol::{temperature::Temperature, temperature::TargetTemperatureRequest, temperature::TemperatureRequest};

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

use tonic::{Request, Response, Status};

use crate::protocol::temperature::temperature_service_server::TemperatureService;
use crate::protocol::{
    temperature::TargetTemperatureRequest, temperature::Temperature,
    temperature::TemperatureRequest,
};

use tracing::{info, instrument};

#[derive(Debug, Default)]
pub struct TemperatureSensor {
    temperature: i32,
    target_temperature: i32,
}

#[tonic::async_trait]
impl TemperatureService for TemperatureSensor {
    #[instrument]
    async fn get_temperature(
        &self,
        request: Request<TemperatureRequest>,
    ) -> Result<Response<Temperature>, Status> {
        info!(
            "Got a (get temperature) request from {:?}",
            request.remote_addr()
        );
        // TODO: Connect to temperature sensor.

        let reply = Temperature {
            temperature: self.temperature,
            target_temperature: self.target_temperature,
        };
        Ok(Response::new(reply))
    }

    #[instrument]
    async fn set_temperature(
        &self,
        request: Request<TargetTemperatureRequest>,
    ) -> Result<Response<Temperature>, Status> {
        info!(
            "Got a (set temperature) request from {:?}",
            request.remote_addr()
        );
        // TODO: Connect to temperature sensor.
        // self.target_temperature = request.get_ref().target_temperature;

        let reply = Temperature {
            temperature: self.temperature,
            target_temperature: self.target_temperature,
        };
        Ok(Response::new(reply))
    }
}

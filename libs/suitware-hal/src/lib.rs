//! Hardware Abstraction Layer for Suitware systems. On debug mode, connects a
//! link to the simulator (over gRPC). On release mode, connects directly to
//! hardware.

use tracing::instrument;

#[derive(Debug, Default)]
pub struct TemperatureSensorHAL;
pub trait TemperatureSensorTrait {
    fn get_temperature(&self) -> i32;

    fn get_target_temperature(&self) -> i32;

    fn set_target_temperature(&self, _target: i32);
}

impl TemperatureSensorTrait for TemperatureSensorHAL {
    #[instrument]
    fn get_temperature(&self) -> i32 {
        if cfg!(not(debug_assertions)) {
            // Running in release mode. Connect to the actual sensor.
            // TODO: We don't have an actual sensor!
            20
        } else {
            use sim::protocol::temperature::temperature_service_client::TemperatureServiceClient;
            use sim::protocol::temperature::TemperatureRequest;
            let mut client = futures::executor::block_on(TemperatureServiceClient::connect(
                "http://[::1]:50061",
            ))
            .unwrap();
            let request = tonic::Request::new(TemperatureRequest {});

            let response = futures::executor::block_on(client.get_temperature(request)).unwrap();

            response.get_ref().temperature
        }
    }

    #[instrument]
    fn get_target_temperature(&self) -> i32 {
        if cfg!(not(debug_assertions)) {
            // Running in release mode. Connect to the actual sensor.
            // TODO: We don't have an actual sensor!
            22
        } else {
            use sim::protocol::temperature::temperature_service_client::TemperatureServiceClient;
            use sim::protocol::temperature::TemperatureRequest;
            let mut client = futures::executor::block_on(TemperatureServiceClient::connect(
                "http://[::1]:50061",
            ))
            .unwrap();
            let request = tonic::Request::new(TemperatureRequest {});

            let response = futures::executor::block_on(client.get_temperature(request)).unwrap();

            response.get_ref().target_temperature
        }
    }

    #[instrument]
    fn set_target_temperature(&self, _target: i32) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_temperature() {
	let temperature = TemperatureSensorHAL {};
	let t = temperature.get_temperature();
	dbg!(t);
    }
}

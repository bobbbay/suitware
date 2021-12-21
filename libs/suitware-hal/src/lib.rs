#[derive(Debug, Default)]
pub struct TemperatureSensorHAL;
pub trait TemperatureSensorTrait {
    fn get_temperature(&self) -> i32;

    fn get_target_temperature(&self) -> i32;

    fn set_target_temperature(&self, _target: i32);
}

impl TemperatureSensorTrait for TemperatureSensorHAL {
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

    fn set_target_temperature(&self, _target: i32) {}
}

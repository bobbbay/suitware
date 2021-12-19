// TODO: Surely, there's a better way to resolve this conflict?
#[cfg(all(feature = "prod", feature = "dev"))]
compile_error!("feature \"prod\" and feature \"dev\" cannot be enabled at the same time");

#[derive(Debug, Default)]
pub struct TemperatureSensorHAL;
pub trait TemperatureSensorTrait {
    fn get_temperature(&self) -> i32;

    fn get_target_temperature(&self) -> i32;

    fn set_target_temperature(&self, _target: i32);
}

#[cfg(feature = "prod")]
impl TemperatureSensorTrait for TemperatureSensorHAL {
    fn get_temperature(&self) -> i32 {
        20
    }

    fn get_target_temperature(&self) -> i32 {
        22
    }

    fn set_target_temperature(&self, _target: i32) {}
}

// TODO: Simulation support is really almost done.
#[cfg(feature = "dev")]
impl TemperatureSensorTrait for TemperatureSensorHAL {
    fn get_temperature(&self) -> i32 {
	use sim::protocol::temperature::temperature_service_client::TemperatureServiceClient;
	use sim::protocol::temperature::TemperatureRequest;
        let mut client = futures::executor::block_on(TemperatureServiceClient::connect("http://[::1]:50061")).unwrap();
	let request = tonic::Request::new(TemperatureRequest {});

	let response = futures::executor::block_on(client.get_temperature(request)).unwrap();

	1
    }

    fn get_target_temperature(&self) -> i32 {
	use sim::protocol::temperature::temperature_service_client::TemperatureServiceClient;
	use sim::protocol::temperature::TemperatureRequest;
        let mut client = futures::executor::block_on(TemperatureServiceClient::connect("http://[::1]:50061")).unwrap();
	let request = tonic::Request::new(TemperatureRequest {});

	let response = futures::executor::block_on(client.get_temperature(request)).unwrap();

	1
    }

    fn set_target_temperature(&self, _target: i32) {}
}

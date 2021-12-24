use futures::{Stream, StreamExt};
use std::pin::Pin;
use tonic::{Request, Response, Status, Streaming};

use crate::protocol::temperature::temperature_service_server::TemperatureService;
use crate::protocol::{
    temperature::TargetTemperatureRequest, temperature::Temperature,
    temperature::TemperatureRequest,
};

use hal::{TemperatureSensorHAL, TemperatureSensorTrait};

use tracing::{debug, instrument};

/// An internal temperature sensor.
#[derive(Debug, Default)]
pub struct TemperatureSensor {
    /// A handle to the HAL object.
    handle: TemperatureSensorHAL,
}

/// A resulting temperature status
type TemperatureResult = Result<Response<Temperature>, Status>;
/// Just a stream of temperatures
type TemperatureStream = Pin<Box<dyn Stream<Item = Result<Temperature, Status>> + Send>>;
/// A resulting streamed temperatures status
type TemperatureStreamResult = Result<Response<TemperatureStream>, Status>;

#[tonic::async_trait]
impl TemperatureService for TemperatureSensor {
    #[instrument]
    async fn get_temperature(&self, request: Request<TemperatureRequest>) -> TemperatureResult {
        debug!("Request from {:?}", request.remote_addr());

        let reply = Temperature {
            temperature: self.handle.get_temperature(),
            target_temperature: self.handle.get_target_temperature(),
        };
        Ok(Response::new(reply))
    }

    #[instrument]
    async fn set_temperature(
        &self,
        request: Request<TargetTemperatureRequest>,
    ) -> TemperatureResult {
        debug!("Request from {:?}", request.remote_addr());

        self.handle
            .set_target_temperature(request.get_ref().target_temperature);

        let reply = Temperature {
            temperature: self.handle.get_temperature(),
            target_temperature: self.handle.get_target_temperature(),
        };
        Ok(Response::new(reply))
    }

    type StreamTemperatureStream = TemperatureStream;

    #[instrument]
    async fn stream_temperature(
        &self,
        request: Request<Streaming<TemperatureRequest>>,
    ) -> TemperatureStreamResult {
        debug!("Request from {:?}", request.remote_addr());

        let mut stream = request.into_inner();

        // TODO: The temperature shouldn't just be captured here.
        let temperature = self.handle.get_temperature();
        dbg!(&temperature);

        let output = async_stream::try_stream! {
            while let Some(note) = stream.next().await {
                let note = note?;
                dbg!(&note);

                let reply = Temperature {
                    temperature,
                    target_temperature: 2,
                };

                yield reply;
            }
        };

        Ok(Response::new(
            Box::pin(output) as Self::StreamTemperatureStream
        ))
    }
}

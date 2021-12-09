use futures::{Stream, StreamExt};
use std::pin::Pin;
use tonic::{Request, Response, Status};

use crate::protocol::temperature::temperature_service_server::TemperatureService;
use crate::protocol::{
    temperature::TargetTemperatureRequest, temperature::Temperature,
    temperature::TemperatureRequest,
};

use tracing::{info, instrument};

#[derive(Debug, Default)]
pub struct TemperatureSensor {
    handle: hal::temperature::TemperatureSensor,
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

        self.handle.get_temperature();

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
    ) -> Result<Response<Temperature>, Status> {
        info!(
            "Got a (set temperature) request from {:?}",
            request.remote_addr()
        );

        self.handle
            .set_target_temperature(request.get_ref().target_temperature);

        let reply = Temperature {
            temperature: self.handle.get_temperature(),
            target_temperature: self.handle.get_target_temperature(),
        };
        Ok(Response::new(reply))
    }

    type StreamTemperatureStream = Pin<Box<dyn Stream<Item = Result<Temperature, Status>> + Send>>;

    #[instrument]
    async fn stream_temperature(
        &self,
        request: tonic::Request<tonic::Streaming<crate::protocol::temperature::TemperatureRequest>>,
    ) -> Result<tonic::Response<Self::StreamTemperatureStream>, tonic::Status> {
        info!("{:?}", request.remote_addr());

        let mut stream = request.into_inner();

        let output = async_stream::try_stream! {
            while let Some(note) = stream.next().await {
                let note = note?;
                // let temperature = self.handle.get_temperature();

                dbg!(&note);

                let reply = Temperature {
                    temperature: 1,
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

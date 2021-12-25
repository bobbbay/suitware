use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use bevy::DefaultPlugins;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
use color_eyre::Result;
use tonic::transport::Server;

pub mod protocol;

use crate::protocol::temperature::temperature_service_server::TemperatureServiceServer;

pub mod systems {
    pub mod temperature {
        use crate::protocol::temperature::temperature_service_server::TemperatureService;
        use crate::protocol::temperature::Temperature;
        use crate::protocol::temperature::{TargetTemperatureRequest, TemperatureRequest};
        use tonic::{Request, Response, Status};

        #[derive(Default)]
        pub struct TemperatureSensor {}

        #[tonic::async_trait]
        impl TemperatureService for TemperatureSensor {
            async fn get_temperature(
                &self,
                _request: Request<TemperatureRequest>,
            ) -> Result<tonic::Response<Temperature>, Status> {
                let reply = Temperature {
                    temperature: 20,
                    target_temperature: 22,
                };

                Ok(Response::new(reply))
            }

            async fn set_temperature(
                &self,
                _request: Request<TargetTemperatureRequest>,
            ) -> Result<Response<Temperature>, Status> {
                let reply = Temperature {
                    temperature: 20,
                    target_temperature: 22,
                };

                Ok(Response::new(reply))
            }
        }
    }
}

#[derive(Debug, Default, Inspectable)]
struct Data;

use crate::systems::temperature::TemperatureSensor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async move {
        let addr = "[::1]:50061".parse().unwrap();
        println!("Simulation server listening on {}", addr);

        let temperature = TemperatureSensor::default();

        Server::builder()
            .add_service(TemperatureServiceServer::new(temperature))
            .serve(addr)
            .await
            .unwrap();
    });
    
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(InspectorPlugin::<Data>::new())
        .run();

    Ok(())
}

pub mod protocol;

use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
use tonic::{transport::Server, Response};

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
        let reply = Temperature {
            temperature: 20,
            target_temperature: 22,
        };

        Ok(Response::new(reply))
    }

    async fn set_temperature(
        &self,
        _request: tonic::Request<protocol::temperature::TargetTemperatureRequest>,
    ) -> Result<tonic::Response<protocol::temperature::Temperature>, tonic::Status> {
        let reply = Temperature {
            temperature: 20,
            target_temperature: 22,
        };

        Ok(Response::new(reply))
    }
}

#[derive(Debug, Default, Inspectable)]
struct Data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50061".parse().unwrap();
    let greeter = TemperatureSensor::default();

    println!("Simulation server listening on {}", addr);

    let server = tokio::spawn(async move {
        Server::builder()
            .add_service(TemperatureServiceServer::new(greeter))
            .serve(addr)
            .await
            .unwrap();
    });

    let bevy = tokio::spawn(async move {
        App::build()
            .add_plugins(DefaultPlugins)
            .add_plugin(InspectorPlugin::<Data>::new())
            .add_system(update.system())
            .run();
    });

    let (server_result, bevy_result) = futures::future::join(server, bevy).await;
    server_result?;
    bevy_result?;

    Ok(())
}

fn update<'a>(data: Res<Data>, _: Query<()>) {
    dbg!();
}

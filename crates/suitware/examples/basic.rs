use color_eyre::Result;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;

use suitware::{Suitware, System, Task};
use tracing::info;

struct TemperatureSensor;

#[async_trait::async_trait]
impl System for TemperatureSensor {
    async fn run(&self) -> Result<(), &dyn std::error::Error> {
        let mqttoptions = MqttOptions::new("temperature-sensor", "localhost", 5001);

        let (client, _) = AsyncClient::new(mqttoptions, 10);

        for i in 1..=100 {
            let serialized = bincode::serialize(&i).unwrap();

	    dbg!();

            client
                .publish("temperature_sensor/get", QoS::AtMostOnce, false, serialized)
                .await
                .unwrap();

	    dbg!();

            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        Ok(())
    }
}

struct FiveMinTask;
impl Task for FiveMinTask {
    fn run(&self) {
        println!("Ran FiveMinTask!");
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    info!("Starting...");

    Suitware::new()
        .bind("")
        .add_system(&TemperatureSensor {})
        .add_task(&FiveMinTask {}, Duration::from_secs(5))
        .start()
        .await?;

    Ok(())
}

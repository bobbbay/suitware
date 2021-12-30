use color_eyre::Result;
use rumqttc::{AsyncClient, MqttOptions, Packet, QoS};
use tokio::time::Duration;

use suitware::{Suitware, System, Task};
use tracing::info;

struct TemperatureSensor;

#[async_trait::async_trait]
impl System for TemperatureSensor {
    async fn run(&self) -> Result<(), &dyn std::error::Error> {
        let mqttoptions = MqttOptions::new("temperature-sensor", "localhost", 5001);

        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 100);

        client
            .subscribe("temperature_sensor/get", QoS::AtMostOnce)
            .await
            .unwrap();

        tokio::spawn(async move {
            for i in 1..=100 {
                let serialized = bincode::serialize(&i).unwrap();

                client
                    .publish("temperature_sensor/get", QoS::AtMostOnce, false, serialized)
                    .await
                    .unwrap();

                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });

        loop {
            let event = eventloop.poll().await.unwrap();
            match event {
                _ => {
                    dbg!(event);
                }
            }
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

use color_eyre::Result;
use rand::Rng;
use rand::SeedableRng;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use tokio::time::Duration;
use tracing::info;

use suitware::{Suitware, System, Task};

struct TemperatureSensor;

#[async_trait::async_trait]
impl System for TemperatureSensor {
    async fn run(&self) -> Result<(), &dyn std::error::Error> {
        let mqttoptions = MqttOptions::new("temperature-sensor", "localhost", 5001);

        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 100);

        tokio::spawn(async move {
            // Prepare a deterministic RNG
            let mut rng = rand_pcg::Pcg32::seed_from_u64(13);

            loop {
                let random_value: f32 = rng.gen::<f32>() * 20.0;
                let serialized = bincode::serialize(&random_value).unwrap();

                client
                    .publish("temperature_sensor/get", QoS::AtMostOnce, false, serialized)
                    .await
                    .unwrap();

                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });

        loop {
            eventloop.poll().await.unwrap();
        }
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

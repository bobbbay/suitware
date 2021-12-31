use std::thread;
use std::thread::sleep;

use color_eyre::Result;
use librumqttd::Broker;
use librumqttd::Config;
use rand::Rng;
use rand::SeedableRng;
use rand_pcg::Lcg64Xsh32;
use rumqttc::EventLoop;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use suitware::NextState;
use suitware::Plugin;
use tokio::time::Duration;
use tracing::info;

use suitware::{Suitware, System, Task};

struct TemperatureSensor {
    client: AsyncClient,
    eventloop: EventLoop,

    rng: Lcg64Xsh32,
}

impl TemperatureSensor {
    fn new() -> Self {
        let options = MqttOptions::new("temperature-sensor", "localhost", 5001);
        let (client, eventloop) = AsyncClient::new(options, 100);

        let rng = rand_pcg::Pcg32::seed_from_u64(13);

        Self {
            client,
            eventloop,
            rng,
        }
    }
}

#[async_trait::async_trait]
impl System for TemperatureSensor {
    async fn run(&mut self) -> Result<NextState, &dyn std::error::Error> {
        let random_value: f32 = self.rng.gen::<f32>() * 20.0;
        let serialized = bincode::serialize(&random_value).unwrap();

        self.client
            .publish("temperature_sensor/get", QoS::AtMostOnce, false, serialized)
            .await
            .unwrap();

        self.eventloop.poll().await.unwrap();

        tokio::time::sleep(Duration::from_secs(1)).await;

        Ok(NextState::Continue)
    }
}

struct FiveMinTask;
impl Task for FiveMinTask {
    fn run(&self) {
        println!("Ran FiveMinTask!");
    }
}

struct MqttPlugin;

impl Plugin for MqttPlugin {
    fn init(&self) {
        let config: Config = confy::load_path("protocol/mqttd.conf").unwrap();
        let mut broker = Broker::new(config);

        thread::spawn(move || {
            broker.start().unwrap();
        });

        // Give the broker sufficient time to start
        sleep(std::time::Duration::from_secs(1));
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    info!("Starting...");

    Suitware::new()
        .add_plugin(Box::new(MqttPlugin {}))
        .add_system(Box::new(TemperatureSensor::new()))
        .add_task(&FiveMinTask {}, Duration::from_secs(5))
        .start()
        .await?;

    Ok(())
}

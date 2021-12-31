//! A basic example that uses the mqtt plugin and systems to create a publishing
//! server.

use suitware::plugins::mqtt::*;

use color_eyre::Result;
use rand::Rng;
use rand::SeedableRng;
use rand_pcg::Lcg64Xsh32;
use rumqttc::QoS;
use suitware::NextState;
use tokio::time::Duration;
use tracing::info;

use suitware::{Suitware, System};

/// A struct for a TemperatureSensor system. All of the properties that it
/// contains are not framework-specific.
struct TemperatureSensor {
    /// An mqtt plugin handle.
    mqtt: MqttClient,

    /// A handle to a deterministic random number generator.
    rng: Lcg64Xsh32,
}

impl TemperatureSensor {
    /// Create a new TemperatureSensor, with default values.
    fn new() -> Self {
        Self {
	    // Here, we create a handle to the mqttclient plugin, and store it
	    // locally.
	    mqtt: MqttClient::new("some-id", "localhost", 5001, 100),
            rng: rand_pcg::Pcg32::seed_from_u64(13),
        }
    }
}

/// A [`System`] implementation for `TemperatureSensor`.
#[async_trait::async_trait]
impl System for TemperatureSensor {
    /// The run() method is given a mutable reference to self. It is run in a
    /// loop. It can determine the next state of itself by return a variant of
    /// [`NextState`].
    async fn run(&mut self) -> Result<NextState, &dyn std::error::Error> {
	// Use the included rng to generate a random f32.
        let random_value: f32 = self.rng.gen::<f32>() * 20.0;

	// Serialize it with bincode, so we can send it over Mqtt.
        let serialized = bincode::serialize(&random_value).unwrap();

	// Publish!
	// Note that since this is run from inside a Tokio runtime, we can use
	// `.await` syntax to start futures.
        self.mqtt.client
            .publish("temperature_sensor/get", QoS::AtLeastOnce, false, serialized)
            .await
            .unwrap();

	// We need to poll the eventloop for `rumqttc` to make sure it sends the
	// message.
        self.mqtt.eventloop.poll().await.unwrap();

	// Let's wait a bit for good measure.
        tokio::time::sleep(Duration::from_secs(1)).await;

	// Let's ask the runner to loop again next time.
        Ok(NextState::Continue)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    info!("Starting...");

    Suitware::new()
        .register_plugin(Box::new(MqttPlugin {}))
        .add_system(Box::new(TemperatureSensor::new()))
        .start()
        .await?;

    Ok(())
}

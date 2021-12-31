//! Suitware plugin for Mqtt based off of `rumqtt`.

use std::thread::{self, sleep};

use librumqttd::{Broker, Config};
use rumqttc::{AsyncClient, EventLoop, MqttOptions};

use crate::Plugin;

/// A `rumqtt`-based mqtt plugin.
pub struct MqttPlugin;

impl Plugin for MqttPlugin {
    /// Create and start a broker on another thread.
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

/// A client to an Mqtt broker.
pub struct MqttClient {
    pub client: AsyncClient,
    pub eventloop: EventLoop,
}

impl MqttClient {
    /// Create a new client based on a basic set of options.
    ///
    /// ```rust
    /// let my_client: MqttClient = MqttClient::new("some-id", "localhost", 5001, 100);
    /// ```
    ///
    /// For more flexibility, use [`rumqttc::MqttOptions`] coupled with [`Self::new_from_options`].
    pub fn new(id: &str, address: &str, port: u16, cap: usize) -> Self {
        let options = MqttOptions::new(id, address, port);
        let (client, eventloop) = AsyncClient::new(options, cap);

        Self { client, eventloop }
    }

    /// Given a set of [`rumqttc::MqttOptions`], create [`Self`].
    /// ```rust
    /// let my_options = MqttOptions::new("some-id", "localhost", 5001);
    /// let my_client = MqttClient::new_from_options(my_options, 100);
    /// ```
    pub fn new_from_options(options: MqttOptions, cap: usize) -> Self {
        let (client, eventloop) = AsyncClient::new(options, cap);
        Self { client, eventloop }
    }
}

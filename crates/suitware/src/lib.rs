use std::{collections::HashMap, thread, time::Duration};

use error::SuitwareError;
use librumqttd::{Broker, Config};
use rumqttc::{AsyncClient, Event::{Incoming, Outgoing}, MqttOptions, Packet, QoS::AtMostOnce};

pub mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum SuitwareError {}
}

pub struct Suitware<'a> {
    tasks: TaskPool<'a>,
}

impl<'a> Suitware<'a> {
    /// Create a new app.
    pub fn new() -> Self {
        Suitware {
            tasks: TaskPool {
                tasks: HashMap::new(),
            },
        }
    }

    /// Bind to a network
    pub fn bind(self, _address: &str) -> Self {
        // TODO: Actually bind
        self
    }

    /// Add a system to the actor
    pub fn add_system(self) -> Self {
        self
    }

    pub fn add_task(mut self, task: &'a dyn Task, duration: Duration) -> Self {
        self.tasks.tasks.insert(duration, task);
        self
    }

    pub async fn start(self) -> Result<(), SuitwareError> {
        self.start_broker().await?;
        Ok(())
    }

    async fn start_broker(&self) -> Result<(), SuitwareError> {
        let config: Config = confy::load_path("protocol/mqttd.conf").unwrap();
        println!("{:#?}", &config);
        let mut broker = Broker::new(config);

        thread::spawn(move || {
            broker.start().unwrap();
        });

        let mqttoptions = MqttOptions::new("some-identifier", "localhost", 5001);

        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        client
            .subscribe("temperature_sensor/get", AtMostOnce)
            .await
            .unwrap();

        tokio::spawn(async move {
            for _ in 1..=100 {
                client
                    .publish("temperature_sensor/get", AtMostOnce, false, "200")
                    .await
                    .unwrap();

                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });

        loop {
            let event = eventloop.poll().await.unwrap();
            match event {
                Incoming(i) => match i {
                    Packet::Publish(p) => println!("{:?}", p.payload),
                    _ => println!("{:?}", i),
                },
                Outgoing(o) => println!("{:?}", o),
            }
        }

        Ok(())
    }
}

#[async_trait::async_trait]
trait System {
    async fn run();
}

struct TaskPool<'a> {
    tasks: HashMap<Duration, &'a dyn Task>,
}

pub trait Task {
    fn run(&self);
}

#[cfg(test)]
mod tests {
    use super::*;

    use color_eyre::Result;

    #[tokio::test]
    async fn create_basic_application() -> Result<()> {
        Suitware::new().start().await?;
        Ok(())
    }
}

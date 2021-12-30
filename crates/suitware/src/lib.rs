use std::{collections::HashMap, thread, time::Duration};

use error::SuitwareError;
use librumqttd::{Broker, Config};
use rumqttc::{AsyncClient, MqttOptions, QoS};

pub mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum SuitwareError {}
}

pub struct Suitware<'a> {
    systems: Vec<&'a dyn System>,
    tasks: TaskPool<'a>,
}

impl<'a> Suitware<'a> {
    /// Create a new app.
    pub fn new() -> Self {
        Suitware {
            systems: vec![],
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
    pub fn add_system(mut self, system: &'a dyn System) -> Self {
        self.systems.push(system);

        self
    }

    pub fn add_task(mut self, task: &'a dyn Task, duration: Duration) -> Self {
        self.tasks.tasks.insert(duration, task);
        self
    }

    pub async fn start(self) -> Result<(), SuitwareError> {
        let config: Config = confy::load_path("protocol/mqttd.conf").unwrap();
        let mut broker = Broker::new(config);

        thread::spawn(move || {
            broker.start().unwrap();
        });

        for system in self.systems {
            system.run().await.unwrap();
        }

        Ok(())
    }
}

#[async_trait::async_trait]
pub trait System {
    async fn run(&self) -> Result<(), &dyn std::error::Error>;
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

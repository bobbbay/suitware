use std::{collections::HashMap, time::Duration};

use error::SuitwareError;

pub mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum SuitwareError {}
}

pub struct Suitware<'a> {
    systems: Vec<Box<dyn System>>,
    tasks: TaskPool<'a>,
    plugins: Vec<Box<dyn Plugin>>,
}

impl<'a> Suitware<'a> {
    /// Create a new, blank app.
    pub fn new() -> Self {
        Suitware {
            systems: vec![],
            tasks: TaskPool {
                tasks: HashMap::new(),
            },
	    plugins: vec![],
        }
    }

    /// Add a system.
    /// ```rust
    /// # use suitware::*;
    /// struct BlankSystem {};
    /// #[async_trait::async_trait]
    /// impl System for BlankSystem { async fn run(&mut self) -> Result<NextState, &dyn std::error::Error> { Ok(NextState::Continue) } }
    /// let app = Suitware::new().add_system(Box::new(BlankSystem {}));
    /// ```
    pub fn add_system(mut self, system: Box<dyn System>) -> Self {
        self.systems.push(system);

        self
    }

    pub fn add_task(mut self, task: &'a dyn Task, duration: Duration) -> Self {
        self.tasks.tasks.insert(duration, task);
        self
    }

    pub fn add_plugin(mut self, plugin: Box<dyn Plugin>) -> Self {
	self.plugins.push(plugin);
	self
    }

    pub async fn start(mut self) -> Result<(), SuitwareError> {
	for plugin in &mut self.plugins {
	    plugin.init();
	}
	
        for system in &mut self.systems {
	    system.init();
	    loop {
		let res = system.run().await.unwrap();
		match res {
		    NextState::Continue => (),
		    NextState::Stop => break,
		}
	    }
	    system.end();
        }

        Ok(())
    }
}

#[async_trait::async_trait]
pub trait System {
    fn init(&self) {}
    async fn run(&mut self) -> Result<NextState, &dyn std::error::Error>;
    fn end(&self) {}
}

pub enum NextState {
    Continue,
    Stop,
}

struct TaskPool<'a> {
    tasks: HashMap<Duration, &'a dyn Task>,
}

pub trait Task {
    fn run(&self);
}

pub trait Plugin {
    fn init(&self);
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

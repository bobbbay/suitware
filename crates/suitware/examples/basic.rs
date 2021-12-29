use color_eyre::Result;
use std::time::Duration;

use suitware::{Suitware, Task};
use tracing::info;

struct TemperatureSensor;

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
        .add_system()
        .add_task(&FiveMinTask {}, Duration::from_secs(5))
        .start()
        .await?;

    Ok(())
}

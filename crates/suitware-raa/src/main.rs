use color_eyre::Result;
use rumqttc::{AsyncClient, MqttOptions, Packet, QoS};
use tracing::instrument;
use tracing_subscriber::util::SubscriberInitExt;

sixtyfps::include_modules!();

/// Initialize error reporter and tracing
#[instrument]
fn install_tracing() -> Result<()> {
    color_eyre::install()?;

    tracing_subscriber::registry().try_init()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    install_tracing()?;

    let handle = Main::new();
    let handle_weak = handle.as_weak();

    let _thread = tokio::spawn(async move {
        let mqttoptions = MqttOptions::new("some-system-id", "localhost", 5001);

        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        client
            .subscribe("temperature_sensor/get", QoS::AtMostOnce)
            .await
            .unwrap();

        loop {
            let event = eventloop.poll().await.unwrap();
            if let rumqttc::Event::Incoming(Packet::Publish(p)) = event {
                let temperature = bincode::deserialize(&p.payload).unwrap();

                // Forward the temperature to the main thread
                handle_weak.clone().upgrade_in_event_loop(move |handle| {
                    handle.set_temperature(temperature);
                });
            }
        }
    });

    handle.run();

    Ok(())
}

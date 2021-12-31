use plotters::prelude::*;

use color_eyre::Result;
use rumqttc::{AsyncClient, MqttOptions, Packet, QoS};
use sixtyfps::{Image, ModelHandle, SharedPixelBuffer, VecModel};
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

    handle.on_render_temperature_sensor(render_temperature_plot);

    let _thread = tokio::spawn(async move {
        let mqttoptions = MqttOptions::new("raa-subscriber", "localhost", 5001);

        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 100);

        client
            .subscribe("temperature_sensor/get", QoS::AtMostOnce)
            .await
            .unwrap();

        loop {
            let event = eventloop.poll().await.unwrap();
            if let rumqttc::Event::Incoming(Packet::Publish(p)) = event {
                let temperature = bincode::deserialize(&p.payload).unwrap();

                handle_weak.clone().upgrade_in_event_loop(move |handle| {
                    handle
                        .get_previous_temperatures()
                        .0
                        .unwrap()
                        .as_any()
                        .downcast_ref::<VecModel<f32>>()
                        .unwrap()
                        .push(temperature);

                    handle.set_temperature(temperature);
                });
            }
        }
    });

    handle.run();

    Ok(())
}

fn render_temperature_plot(previous_temperatures: ModelHandle<f32>, _new_temperature: f32) -> Image {
    let mut buffer = SharedPixelBuffer::new(640, 480);
    let size = (buffer.width() as u32, buffer.height() as u32);

    let backend = BitMapBackend::with_buffer(buffer.make_mut_bytes(), size);

    let root = backend.into_drawing_area();

    root.fill(&WHITE).expect("Error filling drawing area");

    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(0_f32..40_f32, 0_f32..30_f32)
        .unwrap()
        .set_secondary_coord(0_usize..25_usize, 0_usize..12_usize);

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()
        .unwrap();

    let mut data = vec![];
    for i in 0..previous_temperatures.clone().0.unwrap().row_count() {
        data.push((
            i as f32,
            previous_temperatures.clone().0.unwrap().row_data(i),
        ));
    }

    chart
        .draw_series(AreaSeries::new(data, 0., &BLUE.mix(0.2)).border_style(&BLUE))
        .unwrap();

    root.present().expect("error presenting");
    drop(chart);
    drop(root);

    Image::from_rgb8(buffer)
}

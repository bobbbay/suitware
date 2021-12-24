mod temperature;

use color_eyre::Result;

use druid::{widget::Flex, AppLauncher, Widget, WindowDesc};

use temperature::TemperatureState;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize error reporter and tracing
    color_eyre::install()?;

    // Create the window
    let main_window = WindowDesc::new(make_ui);
    let state = TemperatureState { temperature: 0 };
    let launcher = AppLauncher::with_window(main_window);

    let sink = launcher.get_external_handle();
    tokio::spawn(temperature::set_temperature(sink));

    launcher.use_simple_logger().launch(state)?;

    Ok(())
}

fn make_ui() -> impl Widget<TemperatureState> {
    let mut app = Flex::row();

    app.add_child(temperature::Temperature);

    app
}

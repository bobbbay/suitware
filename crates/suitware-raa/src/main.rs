mod temperature;

use druid::{
    widget::Flex,
    AppLauncher, Color, Data, Env, Event, EventCtx, FontDescriptor, FontFamily, Lens,
    PlatformError, Selector, Widget, WindowDesc,
};

use temperature::TemperatureState;

#[tokio::main]
async fn main() -> Result<(), PlatformError> {
    // Create the window
    let main_window = WindowDesc::new(make_ui);
    let state = TemperatureState { temperature: 0 };
    let launcher = AppLauncher::with_window(main_window);

    let sink = launcher.get_external_handle();
    tokio::spawn(temperature::set_temperature(sink));

    launcher.use_simple_logger().launch(state)
}

fn make_ui() -> impl Widget<TemperatureState> {
    let mut app = Flex::row();

    app.add_child(temperature::Temperature);

    app
}

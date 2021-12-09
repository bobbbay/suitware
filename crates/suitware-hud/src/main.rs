use std::time::Duration;
use tokio::time;
use tonic::Request;

use suitware::protocol::temperature::temperature_service_client::TemperatureServiceClient;

use druid::{AppLauncher, Env, Event, EventCtx, ExtEventSink, PlatformError, Selector, Target, Widget, WidgetExt, WindowDesc};

#[tokio::main]
async fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(make_ui);
    let data = 0;
    let launcher = AppLauncher::with_window(main_window);

    let sink = launcher.get_external_handle();
    tokio::spawn(async_function(sink));

    launcher.use_simple_logger().launch(data)
}

struct MyWindow;

impl Widget<i32> for MyWindow {
    fn event(&mut self, _ctx: &mut EventCtx, event: &Event, data: &mut i32, _env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(UPDATE_TEMPERATURE) => {
                *data = *cmd.get_unchecked(UPDATE_TEMPERATURE);
            }
            _ => (),
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &i32,
        _env: &druid::Env,
    ) {
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &i32,
        data: &i32,
        _env: &druid::Env,
    ) {
        if old_data != data {
            ctx.request_paint();
        }
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        _data: &i32,
        _env: &druid::Env,
    ) -> druid::Size {
        bc.max()
    }

    fn paint(&mut self, _ctx: &mut druid::PaintCtx, data: &i32, _env: &druid::Env) {
        dbg!(data);
    }
}

const UPDATE_TEMPERATURE: Selector<i32> = Selector::new("window.temperature.update");

async fn async_function(sink: ExtEventSink) {
    dbg!();
    let mut client = TemperatureServiceClient::connect("http://[::1]:50051")
        .await
        .unwrap();

    let outbound = async_stream::stream! {
        let mut interval = time::interval(Duration::from_secs(1));

        loop {
            let _time = interval.tick().await;
            let note = suitware::protocol::temperature::TemperatureRequest {};

            yield note;
        }
    };

    let response = client
        .stream_temperature(Request::new(outbound))
        .await
        .expect("");
    let mut inbound = response.into_inner();

    while let Some(note) = inbound.message().await.unwrap() {
        sink.submit_command(UPDATE_TEMPERATURE, note.temperature, Target::Auto)
            .unwrap();
    }
}

fn make_ui() -> impl Widget<i32> {
    MyWindow
        .fix_width(300.0)
        .fix_height(300.0)
        .padding(10.0)
        .center()
}

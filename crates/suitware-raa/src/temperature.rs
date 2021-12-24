use druid::{
    Color, Data, Env, Event, EventCtx, ExtEventSink, FontDescriptor, FontFamily, Lens, Selector,
    Target, Widget,
};
use std::time::Duration;
use suitware::protocol::temperature::temperature_service_client::TemperatureServiceClient;
use tokio::time;
use tonic::Request;

// State
#[derive(Clone, Data, Lens)]
pub struct TemperatureState {
    pub temperature: i32,
}

// Widget
pub struct Temperature;

impl Widget<TemperatureState> for Temperature {
    fn event(
        &mut self,
        _ctx: &mut EventCtx,
        event: &Event,
        data: &mut TemperatureState,
        _env: &Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(UPDATE_TEMPERATURE) => {
                data.temperature = *cmd.get_unchecked(UPDATE_TEMPERATURE);
            }
            _ => (),
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &TemperatureState,
        _env: &druid::Env,
    ) {
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &TemperatureState,
        data: &TemperatureState,
        _env: &druid::Env,
    ) {
        if old_data.temperature != data.temperature {
            ctx.request_paint();
        }
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        _data: &TemperatureState,
        _env: &druid::Env,
    ) -> druid::Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &TemperatureState, env: &druid::Env) {
        dbg!(data.temperature);
        let mut layout =
            druid::TextLayout::<String>::from_text(format!("Temperature: {}", data.temperature));
        layout.set_font(FontDescriptor::new(FontFamily::SERIF).with_size(24.0));
        layout.set_text_color(Color::WHITE);
        layout.rebuild_if_needed(ctx.text(), env);

        layout.draw(ctx, (0., 0.));
    }
}

const UPDATE_TEMPERATURE: Selector<i32> = Selector::new("window.temperature.update");

// Asynchronous task
pub async fn set_temperature(sink: ExtEventSink) {
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

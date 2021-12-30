use suitware_synthesis::{vehicle::*, Synthesis};

use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
use fon::{mono::Mono32, Sink};
use wavy::{Speakers, SpeakersSink};

#[derive(Inspectable, Default)]
struct Data {
    distance: u8,
    // TODO: add more!
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(InspectorPlugin::<Data>::new())
        .add_system(update.system())
        .run();
}

fn update<'a>(data: Res<Data>, _: Query<()>) {
    if !data.is_changed() {
        return;
    }

    // The data has changed - update the sound.
    let vehicle = Vehicle {};
    let mut sound = vehicle.synthesize(Params {
        distance: data.distance,
        velocity: 5,
        state: EngineState::Off,
        rate_of_change: 5,
    });

    // TODO: Play sound
    let mut speakers = Speakers::default();
    let mut speakers: SpeakersSink<'_, Mono32> = futures::executor::block_on(speakers.play());
    speakers.stream(sound.drain());
    dbg!();
}

pub trait Synthesis {
    type Params;
    const S_RATE: u32;
    fn synthesize(&self, params: Self::Params) -> fon::Audio<fon::mono::Mono64>;
}

pub mod vehicle {
    use super::Synthesis;

    pub struct Vehicle {}

    #[derive(Debug)]
    pub struct Params {
        /// Distance, in meters, from the spacesuit's head.
        pub distance: u8,
        pub velocity: u32,
        pub state: EngineState,
        pub rate_of_change: u32,
    }

    // TODO: we can add many more states
    #[derive(Debug)]
    pub enum EngineState {
        Off,
        On,
    }

    impl Synthesis for Vehicle {
        type Params = Params;
        const S_RATE: u32 = 48_000;

        fn synthesize(&self, params: Self::Params) -> fon::Audio<fon::mono::Mono64> {
            use fon::{mono::Mono64, Audio, Sink};
            use twang::{Fc, Mix, Signal, Synth};

            fn sound(params: &mut Params, fc: Fc) -> Signal {
                let pitches = [5., 10., 100.].map(|p| p * (((params.velocity + 1) as f64) / 10.));
                let harmonics = [1., 10.];

                // Parameter-based values
                // volume = (-distance + 11) / 10
                // e.g.   = (-5 + 11) / 10
                let volume = (255. - params.distance as f64) / 255.;

                pitches
                    .iter()
                    .map(|p| {
                        harmonics
                            .iter()
                            .enumerate()
                            .map(|(i, v)| fc.freq(p * (i + 1) as f64).sine().gain(v * volume))
                            .mix()
                    })
                    .mix()
            }

            // Initialize audio with five seconds of silence.
            let mut audio = Audio::<Mono64>::with_silence(Self::S_RATE, Self::S_RATE as usize * 5);
            // Create the synthesizer.
            let mut synth = Synth::new(params, sound);
            // Generate audio samples.
            audio.sink(..).stream(&mut synth);

            // Write synthesized audio to WAV file.
            // TODO: crate::wav::write(audio, "synth.wav").expect("Failed to write WAV file");

            audio
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn synthesize_vehicle() -> Result<(), color_eyre::Report> {
        // Initialize error reporter and tracing
        color_eyre::install()?;
        tracing_subscriber::fmt::init();

        use super::vehicle::*;
        use super::Synthesis;

        let vehicle = Vehicle {};
        let _audio = vehicle.synthesize(Params {
            distance: 0,
            velocity: 10,
            state: EngineState::Off,
            rate_of_change: 5,
        });

        Ok(())
    }
}

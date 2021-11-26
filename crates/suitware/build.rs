extern crate vergen;

use vergen::{Config, vergen};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build protocols into Rust modules
    tonic_build::compile_protos("proto/temperature.proto")?;

    // Grab the version for internal info
    vergen(Config::default())?;

    Ok(())
}

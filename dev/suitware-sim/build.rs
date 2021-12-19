fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build protocols into Rust modules
    tonic_build::compile_protos("proto/temperature.proto")?;

    Ok(())
}

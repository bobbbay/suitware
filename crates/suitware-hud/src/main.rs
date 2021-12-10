use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize error reporter and tracing
    color_eyre::install()?;

    Ok(())
}

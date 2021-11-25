use temperature::TemperatureRequest;
use crate::temperature::temperature_service_client::TemperatureServiceClient;

pub mod temperature {
    tonic::include_proto!("temperature");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TemperatureServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(TemperatureRequest {});

    let response = client.get_temperature(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

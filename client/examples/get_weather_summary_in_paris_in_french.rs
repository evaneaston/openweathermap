use openweathermap_client::models::{City, UnitSystem};
use openweathermap_client::{error::ClientError, Client, ClientOptions};

/// Gets the temperature in °C and description of the weather in Paris right now
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), ClientError> {
    let options = ClientOptions {
        units: UnitSystem::Metric,
        language: "fr".to_string(),
        ..ClientOptions::default() // loads API_KEY env var
    };
    let client = Client::new(options)?;
    let reading = client.fetch_weather(&City::new("Paris", "FR")).await?;

    println!(
        "The temperature and weather in France in French is {}, {}",
        reading.main.temp, reading.weather[0].description
    );
    Ok(())
}

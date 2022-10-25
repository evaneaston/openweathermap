use openweathermap_client::models::{City, CityId, Coord};
use openweathermap_client::{error::ClientError, Client, ClientOptions, Query};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), ClientError> {
    // reuse a client for several readings (must be run in an async context)
    let client = Client::new(ClientOptions::default())?; // loads API_KEY env var

    let v: Vec<Box<dyn Query>> = vec![
        Box::new(City::new("Lages", "BR")),
        Box::new(CityId::new(3369157)),
        Box::new(Coord::new(61.1595054, -45.4409551)),
    ];

    for query in v {
        let weather = client.fetch_weather(query.as_ref()).await?;
        println!("The weather for {} is {:?}", query, weather);
    }
    Ok(())
}

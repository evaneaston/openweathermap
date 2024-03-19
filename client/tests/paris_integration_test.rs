use openweathermap_client::models::{City, UnitSystem};
use openweathermap_client::{error::ClientError, Client, ClientOptions};

#[tokio::test]
async fn paris_integration_test() -> Result<(), ClientError> {
    let options = ClientOptions {
        units: UnitSystem::Metric,
        language: "fr".to_string(),
        ..ClientOptions::default() // loads API_KEY env var
    };
    let client = Client::new(options)?;
    let reading = client.fetch_weather(&City::new("Paris", "FR")).await?;

    println!("{:?}", reading);

    assert_eq!(reading.id, 2988507);
    assert_eq!(reading.name, "Paris");
    assert_eq!(reading.sys.country, Some(String::from("FR")));
    assert!(reading.main.temp > -100.0);
    assert!(48.853 <= reading.coord.lat);
    assert!(reading.coord.lat <= 48.854);
    assert!(2.348 <= reading.coord.lon);
    assert!(reading.coord.lon <= 2.349);

    Ok(())
}

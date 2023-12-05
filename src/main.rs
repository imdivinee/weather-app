use log::{error, info, Level};
use std::io;

const UNIT: &str = "metric";
const API_KEY: &str = ""; // Get a openweather-api key and toss that body boy in here
const USER_AGENT: &str = "Mozilla/5.0 (Linux x86_64; rv:115.0) Gecko/20100101 Firefox/115.0";

async fn get_weather(city: &str) -> Result<(), reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units={}",
        city, API_KEY, UNIT
    );

    let response = reqwest::Client::new()
        .get(&url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .send()
        .await?;

    info!("Attempting to fetch weather data for: {}", city);

    if response.status().is_success() {
        let body = response.text().await?;

        match serde_json::from_str::<serde_json::Value>(&body) {
            Ok(data) => {
                let location = data["name"].as_str().unwrap_or("Unknown");
                let temperature = data["main"]["temp"].as_f64().unwrap_or(0.0);
                let weather_type = data["weather"][0]["description"]
                    .as_str()
                    .unwrap_or("Unknown");

                println!("Location: {}", location);
                println!("Temperature: {} °C", temperature);
                println!("Weather Type: {}", weather_type);
                Ok(())
            }
            Err(err) => {
                error!("Failed to parse JSON response: {}", err);
                Ok(())
            }
        }
    } else {
        error!("City not found.");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    simple_logger::init_with_level(Level::Info).unwrap();

    println!("Enter a city name:");
    let mut city = String::new();
    io::stdin().read_line(&mut city)?;

    match get_weather(city.trim()).await {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("Error: {}", err);
            Err(io::Error::new(io::ErrorKind::Other, "Failed to get weather data"))
        }
    }
}

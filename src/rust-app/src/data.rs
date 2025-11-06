use crate::models::WeatherData;
use once_cell::sync::Lazy;
use std::fs;

/// Global weather data loaded from weather.json
pub static WEATHER_DATA: Lazy<WeatherData> = Lazy::new(|| {
    load_weather_data().expect("Failed to load weather data")
});

/// Load weather data from JSON file
fn load_weather_data() -> Result<WeatherData, Box<dyn std::error::Error>> {
    let data = fs::read_to_string("weather.json")?;
    let weather_data: WeatherData = serde_json::from_str(&data)?;
    Ok(weather_data)
}

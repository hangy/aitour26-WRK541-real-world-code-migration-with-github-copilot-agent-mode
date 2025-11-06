use crate::models::WeatherData;
use once_cell::sync::Lazy;
use std::fs;
use std::path::PathBuf;

/// Global weather data loaded from weather.json
pub static WEATHER_DATA: Lazy<WeatherData> = Lazy::new(|| {
    load_weather_data().expect("Failed to load weather data")
});

/// Load weather data from JSON file
fn load_weather_data() -> Result<WeatherData, Box<dyn std::error::Error>> {
    // Try multiple paths to find weather.json
    let possible_paths = vec![
        PathBuf::from("weather.json"),
        PathBuf::from("../weather.json"),
        PathBuf::from("../../weather.json"),
        PathBuf::from("src/rust-app/weather.json"),
    ];
    
    let mut data_content = None;
    for path in possible_paths {
        if let Ok(content) = fs::read_to_string(&path) {
            println!("Loaded weather data from: {:?}", path);
            data_content = Some(content);
            break;
        }
    }
    
    let data = data_content.ok_or("Could not find weather.json in any expected location")?;
    let weather_data: WeatherData = serde_json::from_str(&data)?;
    Ok(weather_data)
}

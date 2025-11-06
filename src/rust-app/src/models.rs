use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Temperature data with high and low values
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Temperature {
    pub high: i32,
    pub low: i32,
}

/// Type alias for month data: month name -> temperature
pub type MonthData = HashMap<String, Temperature>;

/// Type alias for city data: city name -> month data
pub type CityData = HashMap<String, MonthData>;

/// Type alias for country data: country name -> city data
pub type WeatherData = HashMap<String, CityData>;

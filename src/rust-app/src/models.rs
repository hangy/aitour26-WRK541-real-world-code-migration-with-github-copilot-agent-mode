use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

/// Temperature data with high and low values
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct Temperature {
    /// High temperature in Fahrenheit
    pub high: i32,
    /// Low temperature in Fahrenheit
    pub low: i32,
}

/// Type alias for month data: month name -> temperature
pub type MonthData = HashMap<String, Temperature>;

/// Type alias for city data: city name -> month data
pub type CityData = HashMap<String, MonthData>;

/// Type alias for country data: country name -> city data
pub type WeatherData = HashMap<String, CityData>;

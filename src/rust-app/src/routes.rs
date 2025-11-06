use actix_web::{get, web, HttpResponse, Responder};
use crate::data::WEATHER_DATA;
use utoipa::ToSchema;
use serde::{Deserialize, Serialize};

/// Error response structure
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    /// Error message
    error: String,
    /// Country name (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<String>,
    /// City name (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<String>,
    /// Month name (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    month: Option<String>,
}

/// Root endpoint - redirects to API documentation
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 302, description = "Redirect to API documentation")
    ),
    tag = "General"
)]
#[get("/")]
pub async fn root() -> impl Responder {
    HttpResponse::Found()
        .append_header(("Location", "/docs"))
        .finish()
}

/// Get list of all countries
#[utoipa::path(
    get,
    path = "/countries",
    responses(
        (status = 200, description = "List of all available countries", body = Vec<String>)
    ),
    tag = "Weather Data"
)]
#[get("/countries")]
pub async fn countries() -> impl Responder {
    let country_list: Vec<String> = WEATHER_DATA.keys().cloned().collect();
    HttpResponse::Ok().json(country_list)
}

/// Get monthly average temperature for a specific location
#[utoipa::path(
    get,
    path = "/countries/{country}/{city}/{month}",
    params(
        ("country" = String, Path, description = "Country name (case-sensitive)"),
        ("city" = String, Path, description = "City name (case-sensitive)"),
        ("month" = String, Path, description = "Month name (case-sensitive)")
    ),
    responses(
        (status = 200, description = "Temperature data for the specified location", body = crate::models::Temperature),
        (status = 404, description = "Country, city, or month not found", body = ErrorResponse)
    ),
    tag = "Weather Data"
)]
#[get("/countries/{country}/{city}/{month}")]
pub async fn monthly_average(
    path: web::Path<(String, String, String)>,
) -> impl Responder {
    let (country, city, month) = path.into_inner();
    
    // Try to find the data, return 404 if any level is missing
    match WEATHER_DATA.get(&country) {
        None => HttpResponse::NotFound()
            .json(ErrorResponse {
                error: "Country not found".to_string(),
                country: Some(country),
                city: None,
                month: None,
            }),
        Some(cities) => match cities.get(&city) {
            None => HttpResponse::NotFound()
                .json(ErrorResponse {
                    error: "City not found".to_string(),
                    country: Some(country),
                    city: Some(city),
                    month: None,
                }),
            Some(months) => match months.get(&month) {
                None => HttpResponse::NotFound()
                    .json(ErrorResponse {
                        error: "Month not found".to_string(),
                        country: Some(country),
                        city: Some(city),
                        month: Some(month),
                    }),
                Some(temperature) => HttpResponse::Ok().json(temperature),
            },
        },
    }
}

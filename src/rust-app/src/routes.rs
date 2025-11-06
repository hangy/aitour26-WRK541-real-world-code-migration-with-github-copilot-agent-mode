use actix_web::{get, web, HttpResponse, Responder};
use crate::data::WEATHER_DATA;
use crate::models::Temperature;

/// Root endpoint - redirects to API documentation
#[get("/")]
pub async fn root() -> impl Responder {
    HttpResponse::Found()
        .append_header(("Location", "/docs"))
        .finish()
}

/// Get list of all countries
#[get("/countries")]
pub async fn countries() -> impl Responder {
    let country_list: Vec<String> = WEATHER_DATA.keys().cloned().collect();
    HttpResponse::Ok().json(country_list)
}

/// Get monthly average temperature for a specific location
#[get("/countries/{country}/{city}/{month}")]
pub async fn monthly_average(
    path: web::Path<(String, String, String)>,
) -> impl Responder {
    let (country, city, month) = path.into_inner();
    
    // Try to find the data, return 404 if any level is missing
    match WEATHER_DATA.get(&country) {
        None => HttpResponse::NotFound()
            .json(serde_json::json!({
                "error": "Country not found",
                "country": country
            })),
        Some(cities) => match cities.get(&city) {
            None => HttpResponse::NotFound()
                .json(serde_json::json!({
                    "error": "City not found",
                    "country": country,
                    "city": city
                })),
            Some(months) => match months.get(&month) {
                None => HttpResponse::NotFound()
                    .json(serde_json::json!({
                        "error": "Month not found",
                        "country": country,
                        "city": city,
                        "month": month
                    })),
                Some(temperature) => HttpResponse::Ok().json(temperature),
            },
        },
    }
}

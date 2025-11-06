use actix_web::{get, web, HttpResponse, Responder};
use crate::data::WEATHER_DATA;
use crate::models::Temperature;

/// Root endpoint - redirects to API documentation
#[get("/")]
pub async fn root() -> impl Responder {
    // TODO: Implement redirect to /docs
    HttpResponse::Ok().body("Weather API")
}

/// Get list of all countries
#[get("/countries")]
pub async fn countries() -> impl Responder {
    // TODO: Implement country list
    HttpResponse::Ok().json(Vec::<String>::new())
}

/// Get monthly average temperature for a specific location
#[get("/countries/{country}/{city}/{month}")]
pub async fn monthly_average(
    path: web::Path<(String, String, String)>,
) -> impl Responder {
    let (country, city, month) = path.into_inner();
    
    // TODO: Implement data retrieval with proper error handling
    HttpResponse::Ok().json(Temperature { high: 0, low: 0 })
}

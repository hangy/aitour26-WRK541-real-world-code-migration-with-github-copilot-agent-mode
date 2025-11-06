pub mod data;
pub mod models;
pub mod routes;

use actix_web::{App, HttpServer};

/// Create and configure the Actix web application
pub fn create_app() -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        // Routes will be configured here
}

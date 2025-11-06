pub mod data;
pub mod models;
pub mod routes;

use actix_web::App;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// OpenAPI documentation structure
#[derive(OpenApi)]
#[openapi(
    paths(
        routes::root,
        routes::countries,
        routes::monthly_average,
    ),
    components(
        schemas(models::Temperature, routes::ErrorResponse)
    ),
    tags(
        (name = "General", description = "General API endpoints"),
        (name = "Weather Data", description = "Weather data retrieval endpoints")
    ),
    info(
        title = "Weather API",
        version = "0.1.0",
        description = "Historical weather data API providing temperature information for cities worldwide",
        contact(
            name = "API Support",
        )
    )
)]
pub struct ApiDoc;

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
    let openapi = ApiDoc::openapi();
    
    App::new()
        .service(
            SwaggerUi::new("/docs/{_:.*}")
                .url("/api-docs/openapi.json", openapi.clone())
        )
        .service(routes::root)
        .service(routes::docs_redirect)
        .service(routes::countries)
        .service(routes::monthly_average)
}

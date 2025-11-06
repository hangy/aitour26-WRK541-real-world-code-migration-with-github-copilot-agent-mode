use actix_web::HttpServer;
use weather_api::create_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Weather API server on http://0.0.0.0:8000");
    
    HttpServer::new(|| create_app())
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}

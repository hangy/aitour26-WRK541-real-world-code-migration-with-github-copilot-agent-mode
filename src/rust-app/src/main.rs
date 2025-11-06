use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Weather API server on http://0.0.0.0:8000");
    
    HttpServer::new(|| {
        App::new()
            // Routes will be added here
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

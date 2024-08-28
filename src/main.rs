use actix_web::{web, App, HttpServer, HttpResponse};
use std::env;

async fn get_hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello, World!")
}

async fn post_hello(item: web::Json<String>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Received: {}", item))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(get_hello))
            .route("/hello", web::post().to(post_hello))
    })
    .bind(format!("0.0.0.0:{}", port))?  // Using the port from environment variable
    .run()
    .await
}
